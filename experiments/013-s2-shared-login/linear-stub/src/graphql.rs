use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use async_graphql::{
    Context, EmptySubscription, Error, ErrorExtensions, InputObject, MaybeUndefined, Object,
    Result, Schema, SimpleObject,
};
use chrono::{DateTime, Duration, Utc};

use crate::{IssueFixture, Shared};

pub(crate) type ApiSchema = Schema<Query, Mutation, EmptySubscription>;

pub(crate) struct RequestContext {
    shared: Arc<Shared>,
    gated: AtomicBool,
}

impl RequestContext {
    pub fn new(shared: Arc<Shared>) -> Self {
        Self {
            shared,
            gated: AtomicBool::new(false),
        }
    }

    async fn gate(&self, field: &str, identifier: Option<&str>) -> Result<()> {
        let barrier = self.shared.barrier.lock().await.clone();
        if let Some(barrier) = barrier
            && barrier.matches(field, identifier)
            && !self.gated.swap(true, Ordering::SeqCst)
        {
            barrier.arrive().await?;
        }
        Ok(())
    }
}

#[derive(Clone, SimpleObject)]
#[graphql(name = "WorkflowState")]
struct WorkflowState {
    name: String,
}

#[derive(Clone, SimpleObject)]
#[graphql(name = "Issue")]
struct Issue {
    id: String,
    identifier: String,
    title: String,
    description: Option<String>,
    state: WorkflowState,
    updated_at: DateTime<Utc>,
}

impl From<&IssueFixture> for Issue {
    fn from(issue: &IssueFixture) -> Self {
        Self {
            id: format!("stub:{}", issue.identifier),
            identifier: issue.identifier.clone(),
            title: issue.title.clone(),
            description: issue.description.clone(),
            state: WorkflowState {
                name: issue.state.clone(),
            },
            updated_at: issue.updated_at,
        }
    }
}

fn identifier(id: &str) -> &str {
    id.strip_prefix("stub:").unwrap_or(id)
}

fn not_found() -> Error {
    Error::new("Issue not found or not accessible")
        .extend_with(|_, extensions| extensions.set("code", "ENTITY_NOT_FOUND"))
}

#[derive(SimpleObject)]
#[graphql(name = "PageInfo")]
struct PageInfo {
    has_next_page: bool,
    has_previous_page: bool,
    start_cursor: Option<String>,
    end_cursor: Option<String>,
}

#[derive(SimpleObject)]
#[graphql(name = "IssueEdge")]
struct IssueEdge {
    node: Issue,
    cursor: String,
}

#[derive(SimpleObject)]
#[graphql(name = "IssueConnection")]
struct IssueConnection {
    nodes: Vec<Issue>,
    edges: Vec<IssueEdge>,
    page_info: PageInfo,
}

pub(crate) struct Query;

#[Object]
impl Query {
    async fn issue(&self, ctx: &Context<'_>, id: String) -> Result<Issue> {
        let request = ctx.data_unchecked::<Arc<RequestContext>>();
        let identifier = identifier(&id);
        request.gate("issue", Some(identifier)).await?;
        let data = request.shared.data.lock().await;
        if !data.visible.contains(identifier) {
            return Err(not_found());
        }
        data.issues
            .get(identifier)
            .map(Issue::from)
            .ok_or_else(not_found)
    }

    async fn issues(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 50)] first: i32,
        after: Option<String>,
    ) -> Result<IssueConnection> {
        if !(1..=250).contains(&first) {
            return Err(Error::new("first must be between 1 and 250"));
        }
        if after
            .as_deref()
            .is_some_and(|v| !crate::synthetic_identifier(v))
        {
            return Err(Error::new("invalid cursor"));
        }
        let request = ctx.data_unchecked::<Arc<RequestContext>>();
        request.gate("issues", None).await?;
        let data = request.shared.data.lock().await;
        let visible: Vec<_> = data
            .issues
            .values()
            .filter(|issue| data.visible.contains(&issue.identifier))
            .collect();
        let has_previous_page = visible.iter().any(|issue| {
            after
                .as_ref()
                .is_some_and(|cursor| issue.identifier <= *cursor)
        });
        let mut eligible = visible.into_iter().filter(|issue| {
            after
                .as_ref()
                .is_none_or(|cursor| issue.identifier > *cursor)
        });
        let nodes: Vec<Issue> = eligible
            .by_ref()
            .take(first as usize)
            .map(Issue::from)
            .collect();
        let has_next_page = eligible.next().is_some();
        let edges = nodes
            .iter()
            .map(|node| IssueEdge {
                node: node.clone(),
                cursor: node.identifier.clone(),
            })
            .collect();
        Ok(IssueConnection {
            page_info: PageInfo {
                has_next_page,
                has_previous_page,
                start_cursor: nodes.first().map(|v| v.identifier.clone()),
                end_cursor: nodes.last().map(|v| v.identifier.clone()),
            },
            nodes,
            edges,
        })
    }
}

#[derive(InputObject)]
struct IssueUpdateInput {
    title: Option<String>,
    description: MaybeUndefined<String>,
}

#[derive(SimpleObject)]
struct IssuePayload {
    success: bool,
    issue: Issue,
}

pub(crate) struct Mutation;

#[Object]
impl Mutation {
    async fn issue_update(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: IssueUpdateInput,
    ) -> Result<IssuePayload> {
        let request = ctx.data_unchecked::<Arc<RequestContext>>();
        let identifier = identifier(&id);
        request.gate("issueUpdate", Some(identifier)).await?;
        let mut data = request.shared.data.lock().await;
        if !data.visible.contains(identifier) {
            return Err(not_found());
        }
        let issue = data.issues.get_mut(identifier).ok_or_else(not_found)?;
        let changed = input.title.is_some() || !input.description.is_undefined();
        if let Some(title) = input.title {
            issue.title = title;
        }
        match input.description {
            MaybeUndefined::Value(description) => issue.description = Some(description),
            MaybeUndefined::Null => issue.description = None,
            MaybeUndefined::Undefined => (),
        }
        if changed {
            issue.updated_at = Utc::now().max(issue.updated_at + Duration::milliseconds(1));
        }
        Ok(IssuePayload {
            success: true,
            issue: Issue::from(&*issue),
        })
    }
}

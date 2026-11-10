# 1Password and Linear

For S2, where two Agents share one service login with different limits, for A73, 1Password with
no person present, and for S10, where 1Password is the second secret store. The aim is that the
most anything in these tests can reach is one empty vault and one empty Linear workspace.

## 1Password

1. **Make a vault for the tests**, named `conduit-test`, and put nothing else in it. Your other
   vaults stay out of every test.
2. **Make a service account limited to that vault.** In 1Password on the web, open Developer,
   then Service Accounts, and create one named `conduit-test`. Give it access to
   `conduit-test` only, with read and write. 1Password shows its token once.
3. **Save the token in your own vault**, not in `conduit-test`, as an item named
   `conduit-test service account`. Do not paste it anywhere else. When the engine's 1Password
   provider is ready, the coordinator will ask you to run one command that moves it into the
   engine's keeping; the agents never see it.
4. **For A73's second case**, which reads through the desktop app's session, the coordinator
   asks you first, as its safety rules require.

## Linear

1. **Make a separate workspace**, such as `conduit-test`, on the free plan, with one team and a
   few made-up issues. Nothing real goes in it.
2. **Make an API key** in that workspace: Settings, then Security & access, then Personal API
   keys. Give it the narrowest access Linear offers that still reads and writes issues.
3. **Store the key in `conduit-test`**, as an item named `linear-api-key`, and nowhere else. In S2
   the engine reads it from there through the service account; neither Agent is ever given it.

Linear also runs its own MCP server, signed in through OAuth in a browser. That is the third kind
of sign-in of [ruling 49](../proposals/sign-in/README.md#three-kinds-of-sign-in), and a later
test may use it in place of the key.

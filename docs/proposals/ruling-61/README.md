# Ruling 61: the authority of a Fleet

Status: **ruled by the owner on 2026-11-10** (plan-audit ruling 61). This is the record of a
ruling. Where it and the specification differ, the specification governs.

Group B of the [brief](../projects/brief.md) (where it says "Project", read "Fleet", ruling 60)
sets what a Fleet may hold, who acts for it, what it may grant, and how its grants are revoked. The
coordinator put its thirteen rulings with a recommendation each. The owner: "All as recommended,
though what is in C?"

## The decisions

| # | Decision | Answer |
| --- | --- | --- |
| B1 | Does Fleet authority ship in v0? | **Yes**, as the second spec batch, after the S1 rerun |
| B2 | What a Fleet may hold, and who gives it | Two kinds of Conduit: administration of a Pool other than `system`, never listing register, register-replacement or retire-with-resources; and a bounded right to grant on `access`, never unbounded, never listing lift, keep or withhold, its bound naming none of the excluded operations nor acting for a Fleet nor get-secret. Root, or any actor whose own right covers them, gives them |
| B3 | How an actor uses what a Fleet holds | **Forwarding**: the actor asks the Fleet through its own act-for Conduit; the engine, as the Fleet, makes one onward request through the held Conduit named; two linked records. No ruling is amended |
| B4 | Who acts for a Fleet in v0 | Any Interface root appoints, each with its own token, possibly for several Fleets and then marked as bridging them; any Agent the Fleet owns, for its own Fleet only. People other than the owner wait for v1 |
| B5 | Who appoints | Opening or making a Conduit that lists acting for a Fleet is root's alone; closing follows the ordinary rule |
| B6 | Who drives, sets up or retires an Agent that acts for a Fleet | Root alone; appointing an Agent that others already hold Conduits to is refused, naming them |
| B7 | What a grant made through a Fleet may be | To the Fleet itself (administration of a Pool), to one of its Agents that acts for no Fleet, or to an instance it sets up; targeting a Pool other than `system` or a Resource; never listing what starts model work, get-secret, an operation reserved to root, register, register-replacement or retire-with-resources. A Fleet closes only grants it sent and reconciles only its own and its Agents' invocations |
| B8 | How anyone other than root creates a Conduit | A new operation of `access`, `make.v1`, from a registered template, within the bound, with every condition on opening applying to making; only root makes one whose consumer is root. Amends ruling 44, point 2, for Conduits only |
| B9 | What triggers ruling 54's review | Closing any Conduit a grant was made through, of any delegate; one pending action at root, answered per grant "closed" or "kept", with no expiry; marks computed, so later grants under a closed authority are marked too |
| B10 | Who may keep such a grant | An operation of `access` that a bound can withhold, never used by the grant's own actor nor by the holder of an authority it was granted under, nor through a Fleet |
| B11 | Making an individual revocation stick | An explicit withhold: `conduit close <grant> --withhold` also refuses that Fleet re-making the same consumer and target until lifted by ruling 55's lift; a Fleet never holds withhold or lift. A withhold only refuses and permits nothing |
| B12 | How A78 runs in v0 | Fixed worker and tree slots registered and set up once by root; orders 1 to 3 dispatched by root; orders 4 and 5 by a coordinator Agent of the conduit-core Fleet that root appoints, granting each order's tree through its Fleet's Conduits and driving workers through turns root opened; one queue per Fleet; nothing retires a slot |
| B13 | An Agent others can drive, and secrets | A get-secret Conduit is not given to an Agent that an actor can take turns on, nor a turn to an actor on an Agent that reads a secret; inspection and plans show, one hop deep, what each non-root turn-holder reaches |

## What follows

Batch 2 of the specification, after the S1 rerun. Group C is later work, noted in the brief.

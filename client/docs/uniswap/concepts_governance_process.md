# https://docs.uniswap.org/concepts/governance/process

[Skip to main content](https://docs.uniswap.org/concepts/governance/process#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
    * [Uniswap Overview](https://docs.uniswap.org/concepts/overview)
    * [The Uniswap Protocol](https://docs.uniswap.org/concepts/uniswap-protocol)
    * [Protocol Concepts](https://docs.uniswap.org/concepts/governance/process)
    * [Governance](https://docs.uniswap.org/concepts/governance/process)
      * [Overview](https://docs.uniswap.org/concepts/governance/overview)
      * [Process](https://docs.uniswap.org/concepts/governance/process)
      * [Beginners' Guide to Voting](https://docs.uniswap.org/concepts/governance/guide-to-voting)
      * [Adversarial Circumstances](https://docs.uniswap.org/concepts/governance/adversarial-circumstances)
      * [Glossary](https://docs.uniswap.org/concepts/governance/glossary)
    * [Research](https://docs.uniswap.org/concepts/research)
    * [Resources](https://docs.uniswap.org/concepts/resources)
    * [Glossary](https://docs.uniswap.org/concepts/glossary)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * Governance
  * [Process](https://docs.uniswap.org/concepts/governance/process)


On this page
# Process
This is a living document which represents the current process guidelines for developing and advancing Uniswap Governance Proposals. It was last updated September 2024.
## Tools[​](https://docs.uniswap.org/concepts/governance/process#tools "Direct link to Tools")
Uniswap Governance takes place in several venues. Each serves its own particular purpose.
  1. [_Governance Forum_](https://gov.uniswap.org/)


A Discourse-hosted forum for governance-related discussion. Community members must register for an account before sharing or liking posts. New members must read 4 topics and a combined 15 posts over the course of at least 10 minutes before they may post themselves.
  1. [_Snapshot_](https://snapshot.box/#/s:uniswapgovernance.eth)


A simple voting interface that allows users to signal sentiment off-chain. Votes on Snapshot are weighted by the number of UNI delegated to the address used to vote.
  1. [_Uniswap Agora_](https://vote.uniswapfoundation.org)


The [Uniswap Foundation](https://www.uniswapfoundation.org) supports this voting and delegation interface. [Tally](https://www.tally.xyz/gov/uniswap) is another excellent app that supports proposal creation, delegation, and voting.
## Process[​](https://docs.uniswap.org/concepts/governance/process#process "Direct link to Process")
Below we outline the current Uniswap governance process, detailing where these venues fit in. These processes are subject to change according to feedback from the Uniswap community.
### Phase 1: Request for Comment (RFC)[​](https://docs.uniswap.org/concepts/governance/process#phase-1-request-for-comment-rfc "Direct link to Phase 1: Request for Comment \(RFC\)")
_Timeframe_ : At least 7 days
_Form_ : [Governance Forum](https://gov.uniswap.org/) Post
As a proposer, you should use the RFC phase to introduce the community to your proposal. Your post should detail exactly what you are asking delegates to vote on as well as your rationale for why it is a good idea. You should be prepared to answer questions about your proposal. Willingness to adjust based on community feedback is a hallmark of successful past proposals.
To post a RFC, label your post “RFC - [Your Title Here]”. Prior to moving to Phase 2, give the community at least 7 days to read and comment on the RFC. Please respond to questions in the comments, and take feedback into account in the next iteration of the proposal posted in Phase 2.
### Phase 2: Temperature Check[​](https://docs.uniswap.org/concepts/governance/process#phase-2-temperature-check "Direct link to Phase 2: Temperature Check")
_Timeframe_ : 5 days
_Quorum_ : 10M UNI
_Form_ : [Snapshot Poll](https://snapshot.box/#/s:uniswapgovernance.eth)
The purpose of the Temperature Check is to signal community sentiment on a proposal prior to moving towards an onchain vote.
To create a Temperature Check:
  1. Incorporate the community feedback from the RFC phase into the proposal.
  2. Create and post this version of the proposal in the [Governance Forum](https://gov.uniswap.org/) with the title “Temperature Check — [Your Title Here]”. Include a link to the RFC post. You will update the post to include a link to the Snapshot poll after you’ve posted that.
  3. Create a [Snapshot poll](https://snapshot.box/#/s:uniswapgovernance.eth). The voting options should consist of those which have gained support in the RFC Phase. This poll can be either binary or multiple choice but must include a `No change` option. Set the poll duration to 5 days. Include a link to the Forum Temperature Check post.
  4. Update the Forum post with a link to the Snapshot Poll.


At the end of 5 days, the option with the majority of votes wins. There must be at least 10M UNI `Yes` votes to move onto Phase 3. If the “No change” option wins, the proposal will not move onto the Phase 3.
### Phase 3: Governance Proposal[​](https://docs.uniswap.org/concepts/governance/process#phase-3-governance-proposal "Direct link to Phase 3: Governance Proposal")
_Timeframe_ : 2 day waiting period, 7 day voting period, 2 day timelock
_Threshold_ : 1M UNI
_Quorum_ : 40M UNI votes in favor
Form: [Governance Proposal](https://vote.uniswapfoundation.org/)
![](https://docs.uniswap.org/assets/images/Proposal_Flow-8ca9bfffde99de9627f7b352a5578936.png)
Phase 3 is the final step of the governance process. If this vote passes, then a change will be enacted onchain.
To create an onchain Governance Proposal:
  1. Incorporate any last iterations to your proposal based on feedback prior to posting.
  2. Create a topic in the [Governance Forum](https://gov.uniswap.org/) titled "Governance Proposal — [Your Title Here]" and link to previous forum posts and the Temperature Check Snapshot poll.
  3. Create your proposal. This can be done either through an interface (e.g. [Tally](https://tally.xyz/gov/uniswap)) or through writing the calldata for more complicated proposal logic. If the proposal passed, this calldata will execute. If writing the calldata yourself, please review the logic with a qualified Uniswap community member prior to posting the proposal.
  4. Ensure that at least 1 million UNI is delegated to your address in order to submit a proposal, or find a delegate who has enough delegated UNI to meet the proposal threshold to propose on your behalf.
  5. Once you submit the proposal, a two-day voting delay will start. After the voting delay finishes, a ~seven-day voting period begins. If the proposal passes, a two-day timelock must pass before you can execute the proposed code.


## Changes to the Governance Process[​](https://docs.uniswap.org/concepts/governance/process#changes-to-the-governance-process "Direct link to Changes to the Governance Process")
Timeframe: 7 days
_Quorum_ : 40M UNI
Form: [Snapshot Poll](https://snapshot.box/#/s:uniswapgovernance.eth)
In the future, the community governance process above may need to undergo additional changes to continue to meet the needs of the Uniswap community. While an onchain vote is not required to change the majority of this process, a clear display of community support and acceptance is important for process changes to have legitimacy.
Thus, changes to all off-chain community governance processes should be voted on through an off-chain Snapshot vote. There should be a 7-day voting period and 40M UNI quorum.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/concepts/governance/02-process.md)
Was this helpful?
[PreviousOverview](https://docs.uniswap.org/concepts/governance/overview)[NextBeginners' Guide to Voting](https://docs.uniswap.org/concepts/governance/guide-to-voting)
On this page
  * [Tools](https://docs.uniswap.org/concepts/governance/process#tools)
  * [Process](https://docs.uniswap.org/concepts/governance/process#process)
    * [Phase 1: Request for Comment (RFC)](https://docs.uniswap.org/concepts/governance/process#phase-1-request-for-comment-rfc)
    * [Phase 2: Temperature Check](https://docs.uniswap.org/concepts/governance/process#phase-2-temperature-check)
    * [Phase 3: Governance Proposal](https://docs.uniswap.org/concepts/governance/process#phase-3-governance-proposal)
  * [Changes to the Governance Process](https://docs.uniswap.org/concepts/governance/process#changes-to-the-governance-process)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/concepts/governance/02-process.md)
## Footer
[Uniswap Labs](https://docs.uniswap.org/)
### Developers
  * [Dev Chat](https://discord.com/invite/uniswap)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)
  * [Whitepaper](https://app.uniswap.org/whitepaper-v4.pdf)


### Ecosystem
  * [Uniswap App](https://app.uniswap.org/)
  * [Governance](https://www.uniswapfoundation.org/governance)
  * [Blog](https://blog.uniswap.org/)


### Company
  * [Careers](https://boards.greenhouse.io/uniswaplabs)
  * [Brand Assets](https://github.com/Uniswap/brand-assets/raw/main/Uniswap%20Brand%20Assets.zip)
  * [Terms of Service](https://support.uniswap.org/hc/en-us/articles/30935100859661-Uniswap-Labs-Terms-of-Service)
  * [Privacy Policy](https://support.uniswap.org/hc/en-us/articles/30934457771405-Uniswap-Labs-Privacy-Policy)
  * [Trademark Policy](https://support.uniswap.org/hc/en-us/articles/30934762216973-Uniswap-Labs-Trademark-Guidelines)


### Need Help?
  * [Help Center](https://support.uniswap.org/)
  * [Contact Us](https://support.uniswap.org/hc/en-us/requests/new)


@2025 Uniswap Labs
[](https://github.com/uniswap/uniswap-docs)[](https://twitter.com/Uniswap)[](https://discord.com/invite/uniswap)

# https://docs.uniswap.org/concepts/governance/glossary

[Skip to main content](https://docs.uniswap.org/concepts/governance/glossary#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
    * [Uniswap Overview](https://docs.uniswap.org/concepts/overview)
    * [The Uniswap Protocol](https://docs.uniswap.org/concepts/uniswap-protocol)
    * [Protocol Concepts](https://docs.uniswap.org/concepts/protocol/hooks)
    * [Governance](https://docs.uniswap.org/concepts/governance/overview)
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
  * [Glossary](https://docs.uniswap.org/concepts/governance/glossary)


# Glossary
  * **UNI:** An ERC-20 token that designates the weight of a user's voting rights. The more UNI a user has in their wallet, the more weight their delegation or vote on a proposal holds.
  * **Delegation:** UNI holders cannot vote or create proposals until they delegate their voting rights to an address. Delegation can be given to one address at a time, including the holder's own address. Note that delegation does not lock tokens; it simply adds votes to the chosen delegation address.
  * **Proposal:** A proposal is executable code that modifies the governance contract or treasury and how they work. In order to create a proposal, a user must have at least 0.25% (2.5M UNI) of all UNI delegated to their address. Proposals are stored in the "proposals" mapping of the Governor smart contract. All proposals are subject to a 7-day voting period. If the proposer does not maintain their vote weight balance throughout the voting period, the proposal may be canceled by anyone.
  * **Quorum:** In order for a vote to pass, it must achieve quorum of 4% of all UNI (40M) voting in the affirmative. The purpose of the quorum is to ensure that the only measures that pass have adequate voter participation.
  * **Voting:** Users can vote for or against single proposals once they have voting rights delegated to their address. Votes can be cast while a proposal is in the "Active" state. Votes can be submitted immediately using "castVote" or submitted later with "castVoteBySig" (For more info on castVoteBySig and offline signatures, see EIP-712). If the majority of votes (and a 4% quorum of UNI) vote for a proposal, the proposal may be queued in the Timelock.
  * **Voting Period:** Once a proposal has been put forward, Uniswap community members will have a seven day period (the Voting Period) to cast their votes.
  * **Timelock:** All governance and other administrative actions are required to sit in the Timelock for a minimum of 2 days, after which they can be implemented.


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/concepts/governance/05-glossary.md)
Was this helpful?
[PreviousAdversarial Circumstances](https://docs.uniswap.org/concepts/governance/adversarial-circumstances)[NextResearch](https://docs.uniswap.org/concepts/research)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/concepts/governance/05-glossary.md)
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

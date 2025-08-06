# https://docs.uniswap.org/concepts/governance/overview

[Skip to main content](https://docs.uniswap.org/concepts/governance/overview#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
    * [Uniswap Overview](https://docs.uniswap.org/concepts/overview)
    * [The Uniswap Protocol](https://docs.uniswap.org/concepts/uniswap-protocol)
    * [Protocol Concepts](https://docs.uniswap.org/concepts/governance/overview)
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
  * [Overview](https://docs.uniswap.org/concepts/governance/overview)


On this page
# Overview
## Code[​](https://docs.uniswap.org/concepts/governance/overview#code "Direct link to Code")
[`governance`](https://github.com/Uniswap/governance)
## UNI Address[​](https://docs.uniswap.org/concepts/governance/overview#uni-address "Direct link to UNI Address")
`UNI` is deployed at `0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984` on the Ethereum [mainnet](https://etherscan.io/address/0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984), and the [Ropsten](https://ropsten.etherscan.io/address/0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984), [Rinkeby](https://rinkeby.etherscan.io/address/0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984), [Görli](https://goerli.etherscan.io/address/0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984), and [Kovan](https://kovan.etherscan.io/address/0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984) testnets. It was built from commit [ab22c08](https://github.com/Uniswap/governance/commit/ab22c084bacb2636a1aebf9759890063eb6e4946).
### ABI[​](https://docs.uniswap.org/concepts/governance/overview#abi "Direct link to ABI")
```
import Uni from'@uniswap/governance/build/Uni.json'
```

<https://unpkg.com/@uniswap/governance@1.0.2/build/Uni.json>
## Timelock[​](https://docs.uniswap.org/concepts/governance/overview#timelock "Direct link to Timelock")
`Timelock` is deployed at `0x1a9C8182C09F50C8318d769245beA52c32BE35BC` on the Ethereum [mainnet](https://etherscan.io/address/0x1a9C8182C09F50C8318d769245beA52c32BE35BC), and the [Ropsten](https://ropsten.etherscan.io/address/0x1a9C8182C09F50C8318d769245beA52c32BE35BC), [Rinkeby](https://rinkeby.etherscan.io/address/0x1a9C8182C09F50C8318d769245beA52c32BE35BC), [Görli](https://goerli.etherscan.io/address/0x1a9C8182C09F50C8318d769245beA52c32BE35BC), and [Kovan](https://kovan.etherscan.io/address/0x1a9C8182C09F50C8318d769245beA52c32BE35BC) testnets. It was built from commit [ab22c08](https://github.com/Uniswap/governance/commit/ab22c084bacb2636a1aebf9759890063eb6e4946).
### ABI[​](https://docs.uniswap.org/concepts/governance/overview#abi-1 "Direct link to ABI")
```
import Timelock from'@uniswap/governance/build/Timelock.json'
```

<https://unpkg.com/@uniswap/governance@1.0.2/build/Timelock.json>
## GovernorAlpha (Deprecated)[​](https://docs.uniswap.org/concepts/governance/overview#governoralpha-deprecated "Direct link to GovernorAlpha \(Deprecated\)")
`GovernorAlpha` is deployed at `0x5e4be8Bc9637f0EAA1A755019e06A68ce081D58F` on the Ethereum [mainnet](https://etherscan.io/address/0x5e4be8Bc9637f0EAA1A755019e06A68ce081D58F), and the [Ropsten](https://ropsten.etherscan.io/address/0x5e4be8Bc9637f0EAA1A755019e06A68ce081D58F), [Rinkeby](https://rinkeby.etherscan.io/address/0x5e4be8Bc9637f0EAA1A755019e06A68ce081D58F), [Görli](https://goerli.etherscan.io/address/0x5e4be8Bc9637f0EAA1A755019e06A68ce081D58F), and [Kovan](https://kovan.etherscan.io/address/0x5e4be8Bc9637f0EAA1A755019e06A68ce081D58F) testnets. It was built from commit [ab22c08](https://github.com/Uniswap/governance/commit/ab22c084bacb2636a1aebf9759890063eb6e4946).
### ABI[​](https://docs.uniswap.org/concepts/governance/overview#abi-2 "Direct link to ABI")
The `GovernorAlpha` ABI is viewable on [Etherscan](https://etherscan.io/address/0x5e4be8Bc9637f0EAA1A755019e06A68ce081D58F), as well as via an [npm package](https://www.npmjs.com/package/@uniswap/governance).
<https://unpkg.com/@uniswap/governance@1.0.2/build/GovernorAlpha.json>
```
import GovernorAlpha from'@uniswap/governance/build/GovernorAlpha.json'
```

## GovernorAlpha v2 (Deprecated)[​](https://docs.uniswap.org/concepts/governance/overview#governoralpha-v2-deprecated "Direct link to GovernorAlpha v2 \(Deprecated\)")
`GovernorAlpha v2` is deployed at `0xC4e172459f1E7939D522503B81AFAaC1014CE6F6` on the Ethereum [mainnet](https://etherscan.io/address/0xC4e172459f1E7939D522503B81AFAaC1014CE6F6).
### ABI[​](https://docs.uniswap.org/concepts/governance/overview#abi-3 "Direct link to ABI")
The `GovernorAlpha v2` ABI is viewable on [Etherscan](https://etherscan.io/address/0xC4e172459f1E7939D522503B81AFAaC1014CE6F6)
## GovernorBravo (Current)[​](https://docs.uniswap.org/concepts/governance/overview#governorbravo-current "Direct link to GovernorBravo \(Current\)")
`GovernorBravo` is deployed at `0x408ED6354d4973f66138C91495F2f2FCbd8724C3` on the Ethereum [mainnet](https://etherscan.io/address/0x408ED6354d4973f66138C91495F2f2FCbd8724C3#code).
### ABI[​](https://docs.uniswap.org/concepts/governance/overview#abi-4 "Direct link to ABI")
The Governor Bravo ABI can be found on [Etherscan](https://etherscan.io/address/0x408ED6354d4973f66138C91495F2f2FCbd8724C3#code).
## Miscellaneous Addresses[​](https://docs.uniswap.org/concepts/governance/overview#miscellaneous-addresses "Direct link to Miscellaneous Addresses")
**The following addresses only exist on the Ethereum mainnet.**
The UNI merkle distributor address is `0x090D4613473dEE047c3f2706764f49E0821D256e`.
The staking rewards factory address is `0x3032Ab3Fa8C01d786D29dAdE018d7f2017918e12`.
The four staking rewards addresses are:
```
0x6c3e4cb2e96b01f4b866965a91ed4437839a121a0x7fba4b8dc5e7616e59622806932dbea72537a56b0xa1484c3aa22a66c62b77e0ae78e15258bd0cb7110xca35e32e7926b96a9988f61d510e038108d8068e
```

The four year-long vesting contract addresses are:
```
0x4750c43867ef5f89869132eccf19b9b6c4286e1a0xe3953d9d317b834592ab58ab2c7a6ad22b54075d0x4b4e140d1f131fdad6fb59c13af796fd194e41350x3d30b1ab88d487b0f3061f40de76845bec3f1e94
```

The `feeToSetterVester` address is `0x18e433c7Bf8A2E1d0197CE5d8f9AFAda1A771360`.
The `feeTo` address is `0xDAF819c2437a82f9e01f6586207ebF961a7f0970`.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/concepts/governance/01-overview.md)
Was this helpful?
[PreviousToken Integration Issues](https://docs.uniswap.org/concepts/protocol/integration-issues)[NextProcess](https://docs.uniswap.org/concepts/governance/process)
On this page
  * [Code](https://docs.uniswap.org/concepts/governance/overview#code)
  * [UNI Address](https://docs.uniswap.org/concepts/governance/overview#uni-address)
    * [ABI](https://docs.uniswap.org/concepts/governance/overview#abi)
  * [Timelock](https://docs.uniswap.org/concepts/governance/overview#timelock)
    * [ABI](https://docs.uniswap.org/concepts/governance/overview#abi-1)
  * [GovernorAlpha (Deprecated)](https://docs.uniswap.org/concepts/governance/overview#governoralpha-deprecated)
    * [ABI](https://docs.uniswap.org/concepts/governance/overview#abi-2)
  * [GovernorAlpha v2 (Deprecated)](https://docs.uniswap.org/concepts/governance/overview#governoralpha-v2-deprecated)
    * [ABI](https://docs.uniswap.org/concepts/governance/overview#abi-3)
  * [GovernorBravo (Current)](https://docs.uniswap.org/concepts/governance/overview#governorbravo-current)
    * [ABI](https://docs.uniswap.org/concepts/governance/overview#abi-4)
  * [Miscellaneous Addresses](https://docs.uniswap.org/concepts/governance/overview#miscellaneous-addresses)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/concepts/governance/01-overview.md)
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

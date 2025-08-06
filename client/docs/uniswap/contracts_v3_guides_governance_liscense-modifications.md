# https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications

[Skip to main content](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications)
      * [Quickstart](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications)
      * [Technical Reference](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications)
      * [Overview](https://docs.uniswap.org/contracts/v3/overview)
      * [Guides](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications)
        * [Set Up Your Local Environment](https://docs.uniswap.org/contracts/v3/guides/local-environment)
        * [Implement A Swap](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications)
        * [Providing Liquidity](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications)
        * [Liquidity Mining](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications)
        * [Implement Flash Swaps](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications)
        * [Governance Proposals](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications)
          * [License Modifications](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications)
      * [Technical Reference](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications)
    * [UniswapX](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications)
    * [Universal Router](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications)
    * [Permit2](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v3 Protocol
  * Guides
  * Governance Proposals
  * [License Modifications](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications)


On this page
# License Modifications
## Licensing[​](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications#licensing "Direct link to Licensing")
Please note that Uniswap V3 is under [BUSL license](https://github.com/Uniswap/v3-core#licensing) until the Change Date, currently 2023-04-01. Exceptions to the license may be specified by Uniswap Governance via Additional Use Grants, which can, for example, allow V3 to be deployed on new chains. Please follow the [Uniswap Governance process](https://gov.uniswap.org/t/community-governance-process/7732) to request a DAO vote for exceptions to the license, or to move up the Change Date.
License changes must be enacted via the [ENS domain](https://ens.domains/) uniswap.eth, which is controlled by Uniswap Governance. This means (among other things) that Governance has the power to associate arbitrary text with any subdomain of the form X.uniswap.eth. Modifications of the Change Date should be specified at v3-core-license-date.uniswap.eth, and Additional Use Grants should be specified at v3-core-license-grants.uniswap.eth. The process for associating text with a subdomain is detailed below:
ENS Subdomain Details & Process 
If the subdomain does not already exist which can be checked [here](https://app.ens.domains/name/uniswap.eth/subdomains), the [`setSubnodeRecord`](https://docs.ens.domains/contract-api-reference/ens#set-subdomain-record) function of the ENS registry should be called with the following arguments:
  * `node`: `namehash('uniswap.eth')` (`0xa2a03459171c76bff45817330c10ef9f8af07011a33005b73b50189bbc7e7132`)
  * `label`: `keccak256('v3-core-license-date')` (`0xee55740591b0fd5d7a28a6edc49567f6ff3febbe942ec0e2fa49ee536595085b`) or `keccak256('v3-core-license-grants')` (`0x15ff9b5bd7642701a10e5ea8fb29c957ffda4854cd028e9f6218506e6b509af2`)
  * `owner`: [`0x1a9C8182C09F50C8318d769245beA52c32BE35BC`](https://etherscan.io/address/0x1a9c8182c09f50c8318d769245bea52c32be35bc), the Uniswap Governance Timelock
  * `resolver`: [`0x4976fb03c32e5b8cfe2b6ccb31c09ba78ebaba41`](https://etherscan.io/address/0x4976fb03c32e5b8cfe2b6ccb31c09ba78ebaba41), the public ENS resolver.
  * `ttl`: `0`


  1. Then, the [`setText`](https://docs.ens.domains/contract-api-reference/publicresolver#set-text-data) function of the public resolver should be called with the following arguments:


  * `node`: `namehash('v3-core-license-date.uniswap.eth')` (`0x0505ec7822d61b4cfb294f137d1a7f0ceedf162f555a4bf2f4be58a07cf266c5`) or `namehash('v3-core-license-grants.uniswap.eth')` (`0xa35d592ec6e5289a387cba1d5f82be794f495bd5a361a1fb314687c6aefea1f4`)
  * `key`: A suitable label, such as `notice`.
  * `value`: The text of the change. Note that text may already be associated with the subdomain in question. If it does, it can be reviewed at the following URLs for either [v3-core-license-date](https://app.ens.domains/name/v3-core-license-date.uniswap.eth/details) or [v3-core-license-grants](https://app.ens.domains/name/v3-core-license-grants.uniswap.eth/details), and appended to as desired.


Note: [`setContentHash`](https://docs.ens.domains/contract-api-reference/publicresolver#set-content-hash) may also be used to associate text with a subdomain, but `setText` is presented above for simplicity.
These contract function calls should then be encoded into a governance proposal, and approved by Uniswap Governance.
## Proposals[​](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications#proposals "Direct link to Proposals")
Proposals are submitted via `GovernorBravoDelegator` @ `0x408ED6354d4973f66138C91495F2f2FCbd8724C3`, a proxy contract currently pointing to the implementation at `0x53a328F4086d7C0F1Fa19e594c9b842125263026`. NPM packages for consuming the governance contract ABIs, and details on previous versions, are available [here](https://docs.uniswap.org/concepts/governance/overview)
Governor Bravo #propose Parameters 
```
/**  * @notice Function used to propose a new proposal. Sender must have delegates above the proposal threshold  * @param targets Target addresses for proposal calls  * @param values Eth values for proposal calls  * @param signatures Function signatures for proposal calls  * @param calldatas Calldatas for proposal calls  * @param description String description of the proposal  * @return Proposal id of new proposal  */functionpropose(address[]memory targets,uint[]memory values,string[]memory signatures,bytes[]memory calldatas,stringmemory description)publicreturns(uint)
```

## Populating Proposal Calldata[​](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications#populating-proposal-calldata "Direct link to Populating Proposal Calldata")
Below is an example of using a scripting environment to generate a proposal. This is for educational purposes only - that example assumes access to a private key with a sufficient amount of delegated UNI to submit a proposal, which is an insecure practice. There are several ways to generate a proposal transaction and submit it to Ethereum; this example should only be used for reference and not in production.
Populating `Propose` Calldata 
```
import{ Contract, ethers }from'ethers'import{ namehash }from'@ethersproject/hash'import{ keccak256 }from'@ethersproject/keccak256'import{ Interface }from'@ethersproject/abi'// note: contract ABIs should be imported via etherscanimport{GOVERNOR_BRAVO_ABI,ENS_REGISTRY_ABI,ENS_PUBLIC_RESOLVER_ABI}from'./utils'constGOVERNOR_BRAVO_ADDRESS:string='0x408ED6354d4973f66138C91495F2f2FCbd8724C3'constENS_REGISTRY_ADDRESS:string='0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e'constPUBLIC_ENS_RESOLVER_ADDRESS:string='0x4976fb03c32e5b8cfe2b6ccb31c09ba78ebaba41'constUNISWAP_GOVERNANCE_TIMELOCK_ADDRESS:string='0x1a9C8182C09F50C8318d769245beA52c32BE35BC'const provider =newethers.providers.JsonRpcProvider('YOUR_RPC_URL_HERE')const signer = provider.getSigner('YOUR_SIGNER_ADDRESS_HERE')// note: setting the subnode record should only take place if the subdomain does not already existconst ensRegistryInterface =newInterface(ENS_REGISTRY_ABI)const setSubnodeRecordCalldata = ensRegistryInterface.encodeFunctionData('setSubnodeRecord',[// node: The parent nodenamehash('uniswap.eth'),// label: The hash of the label specifying the subnodekeccak256('v3-core-license-grants'),// owner: The address of the new ownerUNISWAP_GOVERNANCE_TIMELOCK_ADDRESS,// resolver: The address of the resolverPUBLIC_ENS_RESOLVER_ADDRESS,// ttl: The TTL, i.e., time to live, in seconds0,])const ensPublicResolverInterface =newInterface(ENS_PUBLIC_RESOLVER_ABI)const setTextCalldata = ensPublicResolverInterface.encodeFunctionData('setText',[// node: The node to updatenamehash('v3-core-license-grants.uniswap.eth'),// key: The key to set'[your-projects-additional-use-grant-title]',// value: The text data value to set'[your-additional-use-grant-description]',])// Create a new local instance of the governorBravo contract// Note that in production the abi should be gathered via etherscanconst governorBravo =newContract(GOVERNOR_BRAVO_ADDRESS,GOVERNOR_BRAVO_ABI, provider)// the ordered list of target addresses for calls to be madeconst targets =[ENS_REGISTRY_ADDRESS,PUBLIC_ENS_RESOLVER_ADDRESS]// The ordered list of values to be passed to the calls to be made. i.e., the amount of// ETH values to be transferred within the transaction. as this example does not include// the transferring of any ETH, this list is empty.const values =[0,0]// The ordered list of function signatures to be called. The signatures arguments// are optional, if not provided, the function signature will be inferred from the calldataconst signatures =['','']// The ordered list of calldata to be passed to each call in the proposal. The calldata// in this example takes the place of the function signature arguments.const calldatas =[setSubnodeRecordCalldata, setTextCalldata]// the description of the proposal.const description ='# TITLE ## SECTION_EXPLANATION'asyncfunctionmain(){try{const txResponse: ethers.providers.TransactionResponse =await governorBravo.connect(signer).propose(targets, values, signatures, calldatas, description)console.log(`Proposal transaction sent: ${txResponse.hash}`)await txResponse.wait(1)console.log(`Proposal has been mined at blocknumber: ${txResponse.blockNumber}, transaction hash: ${txResponse.hash}`)}catch(error){console.error(error)}}main().then(()=>console.log('done'))
```

## Helpful Links[​](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications#helpful-links "Direct link to Helpful Links")
  * [Governor Bravo Proxy](https://etherscan.io/address/0x408ED6354d4973f66138C91495F2f2FCbd8724C3#readProxyContract)
  * [Governor Bravo Delegate](https://etherscan.io/address/0x53a328f4086d7c0f1fa19e594c9b842125263026#code)
  * [ENS Subnode Record Update Details](https://github.com/Uniswap/deploy-v3#licensing)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/guides/governance/license-modifications.md)
Was this helpful?
[PreviousThe Final Contract](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/final-contract)[NextOverview](https://docs.uniswap.org/contracts/v3/reference/overview)
On this page
  * [Licensing](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications#licensing)
  * [Proposals](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications#proposals)
  * [Populating Proposal Calldata](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications#populating-proposal-calldata)
  * [Helpful Links](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications#helpful-links)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/guides/governance/license-modifications.md)
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

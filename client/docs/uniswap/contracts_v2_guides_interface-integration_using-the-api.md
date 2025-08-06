# https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api

[Skip to main content](https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/overview)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/concepts/v4-vs-v3)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/quickstart/hooks/setup)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/reference/errors/)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v3/overview)
    * [Smart Wallet](https://docs.uniswap.org/contracts/smart-wallet/overview)
    * [UniswapX](https://docs.uniswap.org/contracts/uniswapx/overview)
    * [Universal Router](https://docs.uniswap.org/contracts/universal-router/overview)
    * [Permit2](https://docs.uniswap.org/contracts/permit2/overview)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v2/overview)
      * [Overview](https://docs.uniswap.org/contracts/v2/overview)
      * [Concepts](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/how-uniswap-works)
        * [Protocol Overview](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/how-uniswap-works)
        * [Core Concepts](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/swaps)
        * [Advanced Topics](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
      * [Guides](https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api)
        * [Interface Integration](https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api)
          * [Using the API](https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api)
          * [Custom Linking](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking)
          * [Iframe Integration](https://docs.uniswap.org/contracts/v2/guides/interface-integration/iframe-integration)
        * [Smart Contract Integration](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/reference/API/overview)
        * [API](https://docs.uniswap.org/contracts/v2/reference/API/overview)
        * [Governance](https://docs.uniswap.org/contracts/v2/reference/Governance/governance-reference)
        * [Smart Contracts](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v1/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v2 Protocol
  * Guides
  * Interface Integration
  * [Using the API](https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api)


On this page
# Using the API
In this guide we will create a web interface that consumes and displays data from the Uniswap Subgraph. The goal is to provide a quick overview of a setup that you can extend to create your own UIs and analytics around Uniswap data.
Many different libraries can be used to create an interface and a connection to the subgraph graphql endpoint, but in this guide we will use [React](https://reactjs.org/) for the interface, and [Apollo Client](https://www.apollographql.com/docs/react/) for sending queries. We'll also be using yarn for dependency management.
### Setup and Installs[​](https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api#setup-and-installs "Direct link to Setup and Installs")
We'll need to create the basic skeleton for the application. We'll use [create-react-app](https://reactjs.org/docs/create-a-new-react-app.html) for this. We'll also add the dependencies we need. Navigate to your root location in your command line and run:
```
yarn create react-app uniswap-democd uniswap-demoyarn add apollo-client apollo-cache-inmemory apollo-link-http graphql graphql-tag @apollo/react-hooksyarn start
```

In your browser you should see the default React app running. In a text editor open `App.js` within `src` and replace the contents with this stripped down boilerplate. We'll add to this as we go.
```
importReactfrom'react'import'./App.css'functionApp(){return<div></div>}exportdefaultApp
```

### Graphql Client[​](https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api#graphql-client "Direct link to Graphql Client")
We need to set up some middleware in order to make requests to the Uniswap subgraph and receive data. To do this we'll use Apollo and create a graphql client to handle this.
  1. Add the imports shown below and instantiate a new client instance. Notice how we use the link to the Uniswap subgraph here.


```
importReactfrom'react'import'./App.css'import{ApolloClient}from'apollo-client'import{InMemoryCache}from'apollo-cache-inmemory'import{HttpLink}from'apollo-link-http'exportconst client =newApolloClient({link:newHttpLink({uri:'https://api.thegraph.com/subgraphs/name/uniswap/uniswap-v2',}),cache:newInMemoryCache(),})functionApp(){return<div></div>}exportdefaultApp
```

  1. We also need to add a context so that Apollo can handle requests properly. In your `index.js` file import the proper provider and wrap the root in it like this:


```
importReactfrom'react'importReactDOMfrom'react-dom'importAppfrom'./App'importregisterServiceWorkerfrom'./registerServiceWorker'import'./index.css'import{ApolloProvider}from'react-apollo'import{ client }from'./App'ReactDOM.render(<ApolloProvider client={client}><App/></ApolloProvider>,document.getElementById('root'))registerServiceWorker()
```

### Writing the queries[​](https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api#writing-the-queries "Direct link to Writing the queries")
Next we'll construct our query and fetch data. For this example we will fetch some data about the Dai token on Uniswap V2. We'll get the current price, and total liquidity across all pairs. We'll be using the Dai address as an id in this query. We'll also fetch the USD price of ETH to help create USD conversion for Dai data.
  1. First we need to define the query itself. We'll use `gql` to parse a query string into the GraphQL AST standard. Import the `gql` helper into the app and use it to create the query. Add the following to your `App.js` file:


```
importgqlfrom'graphql-tag'constDAI_QUERY= gql`querytokens($tokenAddress:Bytes!){tokens(where:{id:$tokenAddress}){derivedETHtotalLiquidity}}`constETH_PRICE_QUERY= gql`queryethPrice{bundle(id:"1"){ethPrice}}`
```

We use an id of `1` for the bundle because there is only one hardcoded bundle in the subgraph.
### Fetch data[​](https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api#fetch-data "Direct link to Fetch data")
Now we're ready to use these queries to fetch data from the Uniswap V2 subgraph. To do this we can use the `useQuery` hook which uses our client instance to fetch data, and gives us live info about the status of the request. To do this add the following to your `App.js` file:
```
import{ useQuery }from'@apollo/react-hooks'const{ loading, error,data: ethPriceData }=useQuery(ETH_PRICE_QUERY)const{loading: daiLoading,error: daiError,data: daiData,}=useQuery(DAI_QUERY,{variables:{tokenAddress:'0x6b175474e89094c44da98b954eedeac495271d0f',},})
```

Notice we're using the Dai token address to fetch data about Dai.
### Formatting Response[​](https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api#formatting-response "Direct link to Formatting Response")
Now that we have our data we can format it and display it in the UI. First, we parse the return data to get the actual data that we want. Then we'll use it to get the USD price of Dai. Lastly we'll insert this data into the UI itself.
These queries will return a response object for each query. Within each one we're interested in the root field we defined in the query definition. For the `daiData` response we defined this as `tokens`, and for the `ethPriceData` query we defined this as `ethPrice`. Within each one we'll get an array of results. Because we're only querying for single entities we'll reference the `0` index in the data array.
Add the following lines to your `App.js` file to parse the responses:
```
const daiPriceInEth = daiData && daiData.tokens[0].derivedETHconst daiTotalLiquidity = daiData && daiData.tokens[0].totalLiquidityconst ethPriceInUSD = ethPriceData && ethPriceData.bundles[0].ethPrice
```

### Displaying in the UI[​](https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api#displaying-in-the-ui "Direct link to Displaying in the UI")
Finally we can use our parsed response data to hydrate the UI. We'll do this in two steps.
  1. First we'll create loading states. To detect if a query is still pending a response we can reference the loading variables we've already defined. We'll add two loading states, one for the Dai price, and one for the Dai total liquidity. These may flicker fast because the time to query is fast.
  2. Populate with loaded data. Once we detect that the queries have finished loading we can populate the UI with the real data.


To do this add the following lines in the return function of your `App.js` file:
```
return(<div><div>Dai price:{' '}{ethLoading || daiLoading?'Loading token data...':'$'+// parse responses as floats and fix to 2 decimals(parseFloat(daiPriceInEth)*parseFloat(ethPriceInUSD)).toFixed(2)}</div><div>Dai total liquidity:{' '}{daiLoading?'Loading token data...':// display the total amount of DAI spread across all poolsparseFloat(daiTotalLiquidity).toFixed(0)}</div></div>)
```

### Next steps[​](https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api#next-steps "Direct link to Next steps")
This should render a very basic page with these two stats about the Dai token within Uniswap. This is a very basic example of what you can do with the Uniswap subgraph and we encourage you to build out more complex and interesting tools!
You can visit our [analytics site](https://uniswap.info/) to see a more advanced analytics page, or visit [the github](https://github.com/Uniswap/uniswap-info) for more detailed examples of using the Uniswap subgraph to create UIs.
### Review[​](https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api#review "Direct link to Review")
In the end your `App.js` file should look like this:
```
importReact,{ useEffect }from'react'import'./App.css'import{ApolloClient}from'apollo-client'import{InMemoryCache}from'apollo-cache-inmemory'import{HttpLink}from'apollo-link-http'import{ useQuery }from'@apollo/react-hooks'importgqlfrom'graphql-tag'exportconst client =newApolloClient({link:newHttpLink({uri:'https://api.thegraph.com/subgraphs/name/uniswap/uniswap-v2',}),fetchOptions:{mode:'no-cors',},cache:newInMemoryCache(),})constDAI_QUERY= gql`querytokens($tokenAddress:Bytes!){tokens(where:{id:$tokenAddress}){derivedETHtotalLiquidity}}`constETH_PRICE_QUERY= gql`querybundles{bundles(where:{id:"1"}){ethPrice}}`functionApp(){const{loading: ethLoading,data: ethPriceData }=useQuery(ETH_PRICE_QUERY)const{loading: daiLoading,data: daiData }=useQuery(DAI_QUERY,{variables:{tokenAddress:'0x6b175474e89094c44da98b954eedeac495271d0f',},})const daiPriceInEth = daiData && daiData.tokens[0].derivedETHconst daiTotalLiquidity = daiData && daiData.tokens[0].totalLiquidityconst ethPriceInUSD = ethPriceData && ethPriceData.bundles[0].ethPricereturn(<div><div>Dai price:{' '}{ethLoading || daiLoading?'Loading token data...':'$'+// parse responses as floats and fix to 2 decimals(parseFloat(daiPriceInEth)*parseFloat(ethPriceInUSD)).toFixed(2)}</div><div>Dai total liquidity:{' '}{daiLoading?'Loading token data...':// display the total amount of DAI spread across all poolsparseFloat(daiTotalLiquidity).toFixed(0)}</div></div>)}exportdefaultApp
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/guides/interface-integration/01-using-the-api.md)
Was this helpful?
[PreviousResearch](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/research)[NextCustom Linking](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking)
On this page
  * [Setup and Installs](https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api#setup-and-installs)
  * [Graphql Client](https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api#graphql-client)
  * [Writing the queries](https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api#writing-the-queries)
  * [Fetch data](https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api#fetch-data)
  * [Formatting Response](https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api#formatting-response)
  * [Displaying in the UI](https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api#displaying-in-the-ui)
  * [Next steps](https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api#next-steps)
  * [Review](https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api#review)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/guides/interface-integration/01-using-the-api.md)
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

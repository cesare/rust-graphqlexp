import { ApolloClient, InMemoryCache} from "@apollo/client"

const baseUri = process.env.BACKEND_BASE_URI || "http://127.0.0.1:8000"
const client = new ApolloClient({
  uri: `${baseUri}/graphql`,
  cache: new InMemoryCache(),
});

export default client

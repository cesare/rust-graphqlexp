import { ApolloClient, InMemoryCache, gql} from "@apollo/client";
import { GetServerSideProps } from "next";

const client = new ApolloClient({
  uri: "http://127.0.0.1:8000/graphql",
  cache: new InMemoryCache(),
});

type Servant = {
  id: string,
  name: string,
  className: string,
  rarity: number,
};

type Props = {
  servants: Array<Servant>
};

export default function ServantsIndex(props: Props) {
  return <>
    {props.servants.map(servant => (
      <div key={servant.id}>{servant.name} [{servant.className}]</div>
    ))}
  </>
}

export const getServerSideProps: GetServerSideProps = async (context) => {
  const { data } = await client.query({
    query: gql`
      query {
        listServants {
          id name className rarity
        }
      }
    `,
  });

  const servants: Servant[] = data.listServants;
  const props: Props = {
    servants: servants,
  };

  return { props: props }
}

import { ApolloClient, InMemoryCache, gql} from "@apollo/client"
import { GetServerSideProps } from "next"

const client = new ApolloClient({
  uri: "http://127.0.0.1:8000/graphql",
  cache: new InMemoryCache(),
});

type Servant = {
  id: string,
  name: string,
  className: string,
  rarity: number,
  profiles: Profile[],
};

type Profile = {
  text: String,
}

type Props = {
  servant: Servant,
}

export default function ShowServant(props: Props) {
  return <>
    <div>
      <p>{props.servant.name}</p>
      <ul>
        {props.servant.profiles.map(profile => (
          <li>{profile.text}</li>
        ))}
      </ul>
    </div>
  </>
}

export const getServerSideProps: GetServerSideProps = async (context) => {
  const servantId = context.query.id;

  const { data } = await client.query({
    query: gql`
      query fetchServant($servantId: String!) {
        servant(id: $servantId) {
          id name className rarity
          profiles { text }
        }
      }
    `,
    variables: {
      servantId: servantId,
    },
  });

  const servant: Servant = data.servant;
  const props: Props = {
    servant: servant,
  };

  return { props: props }
}

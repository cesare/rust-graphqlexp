import { gql} from "@apollo/client"
import { GetServerSideProps } from "next"

import client from "../../src/backend/client"

type Servant = {
  id: string,
  name: string,
  className: string,
  rarity: number,
  profiles: Profile[],
};

type Profile = {
  id: String,
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
          <li key={profile.id}>{profile.text}</li>
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
          profiles { id text }
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

import { gql } from "@apollo/client"
import { GetServerSideProps } from "next"
import Link from "next/link"

import client from "../../src/backend/client"

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
      <div key={servant.id}>
        <Link href={{pathname: "/servants/[id]", query: {id: servant.id}}}>
          {servant.name}
        </Link>
        [{servant.className}]
      </div>
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

import { GetServerSideProps } from "next";

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
      <div key={servant.id}>{servant.name}</div>
    ))}
  </>
}

export const getServerSideProps: GetServerSideProps = async (context) => {
  const props: Props = {
    servants: [
      {
        id: "dummy-id",
        name: "test",
        className: "caster",
        rarity: 5,
      }
    ]
  };

  return { props: props }
}

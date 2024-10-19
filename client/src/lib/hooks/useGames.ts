import useSWR, { SWRResponse } from "swr";
import { BASE_API_URL } from "@/lib";
import useUser from "@/lib/hooks/useUser";
import { Game } from "@/types";

interface MyGamesRoute {
  message: Game[];
  code: number;
}

export default function useGames() {
  const { authenticated } = useUser();
  const { data, isLoading }: SWRResponse<MyGamesRoute> = useSWR(
    `${BASE_API_URL}/@me/games`,
    async (url) => {
      const res = await fetch(url, {
        credentials: "include",
        mode: "cors",
      });
      const data = await res.json();
      return data;
    },
  );
  return {
    isLoading,
    games: isLoading || !authenticated ? null : data?.message,
  };
}

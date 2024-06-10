import useSWR, { SWRResponse } from "swr";
import { BASE_API_URL } from "@/lib";
import useUser from "@/lib/hooks/useUser";
import { Friend } from "@/types";

interface MyFriendsRoute {
  message: Friend[];
  code: number;
}

export default function useFriends() {
  const { authenticated } = useUser();
  const { data, isLoading }: SWRResponse<MyFriendsRoute> = useSWR(
    `${BASE_API_URL}/@me/friends`,
    async (url) => {
      const res = await fetch(url, {
        credentials: "include",
        mode: "cors",
      });
      const data = await res.json();
      return data;
    }
  );
  return {
    isLoading,
    friends: isLoading || !authenticated ? null : data?.message,
  };
}

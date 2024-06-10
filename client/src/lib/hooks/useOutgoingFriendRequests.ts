import useSWR, { SWRResponse } from "swr";
import { BASE_API_URL } from "@/lib";
import useUser from "@/lib/hooks/useUser";
import { OutgoingFriendRequest } from "@/types";

interface MyOutgoingFriendRequestsRoute {
  message: OutgoingFriendRequest[];
  code: number;
}

export default function useOutgoingFriendRequests() {
  const { authenticated } = useUser();
  const { data, isLoading }: SWRResponse<MyOutgoingFriendRequestsRoute> =
    useSWR(`${BASE_API_URL}/@me/friends/outgoing`, async (url) => {
      const res = await fetch(url, {
        credentials: "include",
        mode: "cors",
      });
      const data = await res.json();
      return data;
    });
  return {
    isLoading,
    outgoingFriendRequests: isLoading || !authenticated ? null : data?.message,
  };
}

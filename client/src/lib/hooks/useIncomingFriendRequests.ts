import useSWR, { SWRResponse } from "swr";
import { BASE_API_URL } from "@/lib";
import useUser from "@/lib/hooks/useUser";
import { IncomingFriendRequest } from "@/types";

interface MyIncomingFriendRequestsRoute {
  message: IncomingFriendRequest[];
  code: number;
}

export default function useIncomingFriendRequests() {
  const { authenticated } = useUser();
  const { data, isLoading }: SWRResponse<MyIncomingFriendRequestsRoute> =
    useSWR(`${BASE_API_URL}/@me/friends/incoming`, async (url) => {
      const res = await fetch(url, {
        credentials: "include",
        mode: "cors",
      });
      const data = await res.json();
      return data;
    });
  return {
    isLoading,
    incomingFriendRequests: isLoading || !authenticated ? null : data?.message,
  };
}

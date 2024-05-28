export const createGame = async (opponent: string) => {
  const res = await fetch("http://localhost:3000/game", {
    credentials: "include",
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      guest: opponent,
    }),
  });
  if (res.status === 201) {
    const { message } = await res.json();
    window.location.href = `/play?gameId=${message.id}`;
  } else {
    alert(`An error occurred: ${JSON.stringify(await res.json())}`);
  }
};

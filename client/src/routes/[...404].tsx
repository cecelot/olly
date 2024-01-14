import { A } from "@solidjs/router";

export default function NotFound() {
  return (
    <main class="text-center mx-auto p-4">
      <h1 class="text-6xl text-gray-100 uppercase my-4">{":("}</h1>
      <h3 class="text-lg text-gray-300">
        The page you are looking for does not exist
      </h3>
      <p class="my-4">
        <A href="/" class="text-green-400 hover:text-green-500 transition-all">
          {"["}Home{"]"}
        </A>
      </p>
    </main>
  );
}

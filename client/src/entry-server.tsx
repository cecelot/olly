import { StartServer, createHandler } from "@solidjs/start/server";

const title = "Othello";
const description =
  "Play Othello with your friends! Powered by Olly, an open-source Othello server.";

export default createHandler(() => (
  <StartServer
    document={({ assets, children, scripts }) => (
      <html lang="en">
        <head>
          <meta charset="utf-8" />
          <meta name="viewport" content="width=device-width, initial-scale=1" />
          <link rel="icon" href="/favicon.ico" />
          <title>{title}</title>
          <meta name="description" content={description} />
          <meta name="og:title" content={title} />
          <meta name="og:description" content={description} />
          <meta name="theme-color" content="#8839ef" />
          {assets}
        </head>
        <body class="latte bg-base">
          <div id="app">{children}</div>
          {scripts}
        </body>
      </html>
    )}
  />
));

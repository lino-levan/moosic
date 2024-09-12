import songs from "./public/song/songs.json" with { type: "json" };
import { serveDir } from "https://deno.land/std@0.210.0/http/file_server.ts";

Deno.serve((req) => {
  const path = new URL(req.url).pathname;

  if (path === "/api/get_song") {
    // Get random song from json file
    const songId = Object.keys(
      songs,
    )[
      Math.floor(Math.random() * Object.keys(songs).length)
    ] as keyof typeof songs;
    return new Response(JSON.stringify({ id: songId, ...songs[songId] }), {
      headers: { "content-type": "application/json" },
    });
  }

  return serveDir(req, {
    fsRoot: "public",
  });
});

const express = require("express");

const app = express();

app.get("/:delay/:message", (req, res) => {
  const { delay, message } = req.params;
  setTimeout(() => res.send(message), delay);
});

app.listen(8000, () => {
  console.log("Server is running on port 8000");
});

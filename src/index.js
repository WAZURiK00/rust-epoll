const express = require("express");

const app = express();

app.get("/:delay/:message", (req, res) => {
    const { delay, message } = req.params;
    console.log(
        `Request received with delay: ${delay / 1000} seconds and message: ${message}`,
    );
    setTimeout(() => res.send(message), delay);
});

app.listen(8000, () => {
    console.log("Server is running on port 8000");
});

const express = require("express");
const fs = require("fs");
const app = express();
app.use(express.json());

function renderFile(req, res, file) {
  fs.readFile(file, "utf8", (err, data) => {
    if (err) {
      res.status(500).send("Erreur serveur");
    } else {
      res.set("Content-Type", "text/html");
      res.send(data);
    }
  });
}

app.get("/", (req, res) => {
  res.send("it's work");
});

app.get("/main-page", (req, res) => {
  console.log("main-page");
  renderFile(req, res, "./pages/main-page.html");
});

app.get("/paiment", (req, res) => {
  console.log("paiment");

  renderFile(req, res, "./pages/paiement.html");
});

app.get("/solde", (req, res) => {
  console.log("solde");

  renderFile(req, res, "./pages/solde.html");
});

app.get("/validate-code", (req, res) => {
  console.log("valid-code");
  console.log(req.query);
  

  renderFile(req, res, "./pages/valid-code.html");
});

app.use((req, res) => {
  res.status(404).send("Page non trouvée");
});

app.listen(8888, () => console.log("Serveur en écoute sur le port 8888"));

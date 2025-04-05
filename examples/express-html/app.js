const express = require("express");
const app = express();
const path = require("path");
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

app.set("view engine", "hbs");
app.set("views", path.join(__dirname, "pages"));

app.get("/", (req, res) => {
  res.send("it's work");
});

app.get("/main-page", (req, res) => {
  console.log("main-page");
  res.render("main-page");
});

app.get("/paiment", (req, res) => {
  console.log("paiment");
  res.render("paiement");
});

app.get("/solde", (req, res) => {
  console.log("solde");
  res.render("solde");
});

app.post("/validate-code", (req, res) => {
  console.log("valid-code");
  console.log(req.body);
  res.render("valid-code");
});

app.get("/form-get", (req, res) => {
  console.log("form-get");
  res.render("form-get");
});

app.get("/handle-form-get", (req, res) => {
  console.log("handle-form-get");
  console.log(req.query);
  const { text } = req.query;
  res.render("handle-form-get", { text });
});

app.get("/form-post", (req, res) => {
  console.log("form-post");
  res.render("form-post");
});

app.post("/handle-form-post", (req, res) => {
  console.log("handle-form-post");
  console.log(req.body);
  const { code } = req.body;
  res.render("handle-form-post", { code });
});

app.get("/not-in-history", (req, res) => {
  console.log("not-in-history");
  res.render("not-in-history");
});

app.get("/after-not-in-history", (req, res) => {
  console.log("after-not-in-history");
  res.render("after-not-in-history");
});

app.get("/end", (req, res) => {
  console.log("end");
  res.render("end");
});

app.use((req, res) => {
  res.status(404).send("Page non trouvée");
});

app.listen(8888, () => console.log("Serveur en écoute sur le port 8888"));

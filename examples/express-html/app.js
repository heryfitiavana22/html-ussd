const express = require("express");
const app = express();
const path = require("path");
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

app.set("view engine", "hbs");
app.set("views", path.join(__dirname, "pages"));

const HOST = process.env.HOST || "http://localhost:8888"

app.get("/", (req, res) => {
  res.send("it's work");
});

app.get("/main-page", (req, res) => {
  console.log("main-page");
  console.log(req.headers);
  console.log(req.query);

  res.render("main-page", {
    host: HOST
  });
});

app.get("/paiment", (req, res) => {
  console.log("paiment");
  console.log(req.query);
  console.log(req.headers);

  res.render("paiement", {
    host: HOST
  });
});

app.get("/solde", (req, res) => {
  console.log("solde");
  res.render("solde", {
    host: HOST
  });
});

app.post("/validate-code", (req, res) => {
  console.log("valid-code");
  console.log(req.body);
  res.render("valid-code", {
    host: HOST
  });
});

app.get("/form-get", (req, res) => {
  console.log("form-get");
  res.render("form-get", {
    host: HOST
  });
});

app.get("/handle-form-get", (req, res) => {
  console.log("handle-form-get");
  console.log(req.query);
  const { text } = req.query;
  res.render("handle-form-get", { text, host: HOST });
});

app.get("/form-post", (req, res) => {
  console.log("form-post");
  res.render("form-post", {
    host: HOST
  });
});

app.post("/handle-form-post", (req, res) => {
  console.log("handle-form-post");
  console.log(req.body);
  console.log(req.headers);
  console.log(req.query);
  const { code } = req.body;
  res.render("handle-form-post", { code,  host: HOST });
});

app.get("/not-in-history", (req, res) => {
  console.log("not-in-history");
  res.render("not-in-history", {
    host: HOST
  });
});

app.get("/after-not-in-history", (req, res) => {
  console.log("after-not-in-history");
  res.render("after-not-in-history", {
    host: HOST
  });
});

app.get("/not-in-cache", (req, res) => {
  console.log("not-in-cache");
  res.render("not-in-cache", {
    host: HOST
  });
});

app.get("/list-offre", (req, res) => {
  console.log("list-offre");
  res.render("list-offre", {
    host: HOST
  });
});

app.get("/handle-offre", (req, res) => {
  console.log("handle-offre");
  console.log(req.query);
  const { user_entry } = req.query;
  res.render("offre-" + user_entry, {
    host: HOST
  });
});

app.get("/end", (req, res) => {
  console.log("end");
  res.render("end");
});

app.use((req, res) => {
  res.status(404).send("Page non trouvée");
});

app.listen(8888, () => console.log("Serveur en écoute sur le port 8888"));

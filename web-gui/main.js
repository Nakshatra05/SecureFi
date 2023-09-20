const express = require("express");
const app = express();
const fs = require("fs");
const { exec } = require("child_process");
const port = 6969;

app.use(express.json()); // to get the data in json format
app.use(express.urlencoded({ extended: true })); // to get the form data

app.set("view engine", "ejs");
app.use(express.static("public")); // to view static files

app.get("/", (req, res) => {
  res.status(200).render("index", { layout: false });
});

app.get("/contract", (req, res) => {
  let contract = "Aave v3";
  res.render("contract", { name: contract });
  console.log("this is cool");
});

app.post("/audit", (req, res) => {
  fs.writeFileSync('C:/Users/Lenovo/Desktop/FinalBNB/SecureFi/bytecode/Exploiter.sol', req.body.code);
  exec("./script.sh", (error, stdout, stderr) => {
    if (error) {
      console.log(`error: ${error.message}`);
      return;
    }
    if (stderr) {
      console.log(`stderr: ${stderr}`);
      return;
    }
    console.log(`stdout: ${stdout}`);
  });
});

app.listen(port, () => {
  console.log(`App listening at port ${port}`);
});

document.addEventListener("DOMContentLoaded", function (event) {
  console.log("hey guys");
  let auditor = document.getElementById("auditor");
  if (window.ethereum) {
    ethereum
      .request({ method: "eth_requestAccounts" })
      .then((accounts) => {
        auditor.innerHTML = `auditor : ${accounts[0]}`;
      })
      .catch((err) => console.error(err.message));
  } else {
    console.error("Install Metamask");
  }

  document.getElementById("c1").addEventListener("click", async () => {
    if (auditor.innerHTML == "auditor : none") {
      alert("Please connect your metamask account");
    }
  });
});

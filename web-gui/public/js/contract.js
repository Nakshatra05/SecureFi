document.addEventListener("DOMContentLoaded", function (event) {
    console.log("hey guys");
    let auditor = document.getElementById("auditor");
    let auditor_address;
    if (window.ethereum) {
      ethereum
        .request({ method: "eth_requestAccounts" })
        .then(async (accounts) => {
          auditor.innerHTML = `auditor : ${accounts[0]}`;
          auditor_address = accounts[0];
        })
        .catch((err) => console.error(err.message));
    } else {
      console.error("Install Metamask");
    }

    document.getElementById("audit").addEventListener("click", () => {
      let text = document.getElementById("auditor_code").value;
      axios.post("/audit", {auditor : auditor_address, code : text});
    })
  });
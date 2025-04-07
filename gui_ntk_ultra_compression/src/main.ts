import { invoke } from "@tauri-apps/api";

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;

async function greet() {
  if (greetMsgEl && greetInputEl) {
    greetMsgEl.textContent = await invoke("greet", {
      name: greetInputEl.value,
    }) as string;
  }
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });

  // Ajout d'un event listener pour le bouton de compression
  document.querySelector("#compress")?.addEventListener("click", async () => {
        try {
            const resultat = await invoke("ma_fonction_rust") as string;
            console.log("Réponse de Rust:", resultat);
            alert("Rust a répondu: " + resultat);
        } catch (error) {
            console.error("Erreur lors de l'appel à Rust:", error);
            alert("Erreur: " + error);
        }
    });
});

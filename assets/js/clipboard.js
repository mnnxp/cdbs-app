
export class Clipboard {
    constructor(value) {
        this.value = value;
    }

    click() {
        console.warn("Clipboard is ok");
        navigator.clipboard.writeText(this.value)
            .then(() => console.log('Text copied.'))
            .catch(() => console.log('Failed to copy text.'));
    }
}

// navigator.clipboard
//   .readText()
//   .then(
//     (clipText) => (document.querySelector(".cliptext").innerText = clipText),
//   );

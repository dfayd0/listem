document.addEventListener("DOMContentLoaded", () => {
    // Select the trigger button and the dialog element
    const addBtn = document.querySelector('.icon-btn.add-btn');
    const dialog = document.querySelector('dialog');
  
    // Select the close button inside the dialog (using its aria-label)
    const closeBtn = dialog.querySelector('button[aria-label="Close"]');
  
    // When the trigger button is clicked, open the dialog
    addBtn.addEventListener('click', () => {
      dialog.showModal();
    });
  
    // When the close button is clicked, close the dialog
    closeBtn.addEventListener('click', () => {
      dialog.close();
    });
  });
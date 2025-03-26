document.addEventListener("DOMContentLoaded", function () {
    const searchInput = document.getElementById("todo-search");
  
    searchInput.addEventListener("input", function () {
      const filter = searchInput.value.toLowerCase();
      // Adjust the selector based on your todo card class (either .unique-card or .todo-card)
      const todos = document.querySelectorAll(".unique-card");
  
      todos.forEach((todo) => {
        // Let's assume the todo title is in an element with the class .card-title
        const titleEl = todo.querySelector(".card-title");
        const descriptionEl = todo.querySelector(".card-description");
  
        const titleText = titleEl ? titleEl.textContent.toLowerCase() : "";
        const descriptionText = descriptionEl ? descriptionEl.textContent.toLowerCase() : "";
  
        // If either title or description includes the filter string, show the todo.
        if (titleText.includes(filter) || descriptionText.includes(filter)) {
          todo.style.display = "";
        } else {
          todo.style.display = "none";
        }
      });
    });
  });
  
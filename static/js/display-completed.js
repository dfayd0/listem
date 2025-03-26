document.addEventListener('DOMContentLoaded', () => {
  const checkbox = document.getElementById('checkbox');
  const todosContainer = document.getElementById('todos');

  // Function to filter completed todos based on checkbox state
  function filterCompletedTodos() {
    const hideCompleted = !checkbox.checked;
    document.querySelectorAll('.completed').forEach(item => {
      if (hideCompleted) {
        item.classList.add('hidden');
      } else {
        item.classList.remove('hidden');
      }
    });
  }

  if (checkbox) {
    // Update filtering when the checkbox state changes
    checkbox.addEventListener('change', filterCompletedTodos);
  }

  if (todosContainer) {
    // Use a MutationObserver to reapply filtering when todos are added or modified
    const observer = new MutationObserver(filterCompletedTodos);
    observer.observe(todosContainer, { childList: true, subtree: true });
  }

  filterCompletedTodos();
});

document.addEventListener('DOMContentLoaded', () => {
    const updateTaskCounts = () => {
      const totalTasks = document.querySelectorAll('.unique-card').length;
      const completedTasks = document.querySelectorAll('.unique-card.completed').length;
      const remainingTasks = totalTasks - completedTasks;
  
      // Update the counts in your footer
      document.getElementById('total-tasks').textContent = "total tasks: " + totalTasks;
      document.getElementById('completed-tasks').textContent = "completed tasks: " + completedTasks;
      document.getElementById('remaining-tasks').textContent = "remaining tasks: " + remainingTasks;
    };
  
    // Update counts on page load
    updateTaskCounts();
  
    // Recalculate counts after any HTMX request
    document.body.addEventListener('htmx:afterRequest', updateTaskCounts);
  });
  
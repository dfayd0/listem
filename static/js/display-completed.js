document.addEventListener('DOMContentLoaded', () => {
    const checkbox = document.getElementById('checkbox');
    if (checkbox) {
      checkbox.addEventListener('change', () => {
        document.querySelectorAll('.completed').forEach(item => {
          item.classList.toggle('hidden');
        });
      });
    }
  });
  
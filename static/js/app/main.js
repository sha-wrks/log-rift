;(function ($, _, Backbone, Handlebars) {

  $(function () {

    App.tasks = new App.Collections.TaskList();

    App.listView = new App.Views.TaskList();

    App.router = new App.Router();

    Backbone.history.start();

    App.router.navigate('', { trigger: true });

    // Theme
    (function () {
      var saved = localStorage.getItem('ordo-theme');
      if (saved === 'dark' || (!saved && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
        document.documentElement.classList.add('dark');
      }
      $('#theme-toggle').on('click', function () {
        var isDark = document.documentElement.classList.toggle('dark');
        localStorage.setItem('ordo-theme', isDark ? 'dark' : 'light');
      });
    }());

    // Command palette
    (function () {
      var $overlay = $('#command-palette-overlay');
      var $input = $('#cmd-input');
      var $results = $('#cmd-results');
      $(document).on('keydown', function (e) {
        if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
          e.preventDefault();
          $overlay.toggleClass('hidden');
          if (!$overlay.hasClass('hidden')) $input.focus();
        }
      });
      $overlay.on('click', function (e) {
        if (e.target === this) $overlay.addClass('hidden');
      });
      $(document).on('keydown', function (e) {
        if (e.key === 'Escape') $overlay.addClass('hidden');
      });
      $results.on('click', '.cmd-item', function () {
        var action = $(this).data('action');
        $overlay.addClass('hidden');
        if (action === 'focus-search') {
          $('#search-input').focus();
        } else if (action === 'add-task') {
          $('#quick-title').focus();
        } else if (action === 'toggle-theme') {
          $('#theme-toggle').click();
        } else if (action === 'go-home') {
          Backbone.history.navigate('', { trigger: true });
        }
      });
      $input.on('input', function () {
        var q = $(this).val().toLowerCase();
        $results.find('.cmd-item').each(function () {
          var text = $(this).text().toLowerCase();
          $(this).toggle(text.indexOf(q) !== -1);
        });
      });
    }());

    // Confetti
    App.confetti = function () {
      var canvas = document.getElementById('confetti-canvas');
      if (!canvas) return;
      var ctx = canvas.getContext('2d');
      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight;
      var pieces = [];
      var colors = ['#6366f1','#10b981','#f59e0b','#ef4444','#ec4899','#8b5cf6'];
      for (var i = 0; i < 120; i++) {
        pieces.push({
          x: Math.random() * canvas.width,
          y: -20 - Math.random() * 300,
          w: 6 + Math.random() * 6,
          h: 4 + Math.random() * 4,
          color: colors[Math.floor(Math.random() * colors.length)],
          vx: (Math.random() - 0.5) * 3,
          vy: 2 + Math.random() * 3,
          rot: Math.random() * 360,
          rv: (Math.random() - 0.5) * 6
        });
      }
      var frames = 0;
      function animate() {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        var alive = false;
        for (var i = 0; i < pieces.length; i++) {
          var p = pieces[i];
          p.x += p.vx;
          p.y += p.vy;
          p.vy += 0.06;
          p.rot += p.rv;
          if (p.y < canvas.height + 50) {
            alive = true;
            ctx.save();
            ctx.translate(p.x, p.y);
            ctx.rotate(p.rot * Math.PI / 180);
            ctx.fillStyle = p.color;
            ctx.fillRect(-p.w / 2, -p.h / 2, p.w, p.h);
            ctx.restore();
          }
        }
        frames++;
        if (alive && frames < 200) requestAnimationFrame(animate);
      }
      animate();
    };

    // Keyboard shortcuts
    $(document).on('keydown', function (e) {
      var tag = e.target.tagName;
      if (tag === 'INPUT' || tag === 'TEXTAREA' || tag === 'SELECT') {
        if (e.key === 'Escape') {
          e.target.blur();
          $('#search-clear').click();
        }
        return;
      }
      if (e.key === '/' && !e.metaKey && !e.ctrlKey) {
        e.preventDefault();
        $('#search-input').focus();
      }
      if (e.key === 'n' && !e.metaKey && !e.ctrlKey) {
        e.preventDefault();
        $('#quick-title').focus();
      }
    });

    // Notify close
    $(document).on('click', '.notify-close', function () {
      $(this).closest('[role="alert"]').remove();
    });

  });

})(jQuery, _, Backbone, Handlebars);

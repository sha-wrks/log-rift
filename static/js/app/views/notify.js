;(function () {

  var template = Handlebars.compile($('#tpl-notify').html());

  App.Views.Notify = {};

  App.Views.Notify.success = function (msg) {
    App.Views.Notify._show(msg, 'success', 'fa-check-circle');
  };

  App.Views.Notify.error = function (msg) {
    App.Views.Notify._show(msg, 'danger', 'fa-exclamation-triangle');
  };

  App.Views.Notify.undo = function (msg, onUndo) {
    var html = Handlebars.compile($('#tpl-notify-undo').html())({ message: msg });
    $('#notify-region').html(html);
    App.Animations.slideIn('#notify-region [role="alert"]', 'down');
    var undone = false;
    var $alert = $('#notify-region [role="alert"]');
    $alert.find('.notify-undo').on('click', function () {
      undone = true;
      onUndo();
      gsap.to($alert[0], { opacity: 0, y: -12, duration: 0.2, onComplete: function () { $alert.remove(); } });
    });
    setTimeout(function () {
      if (!undone) {
        gsap.to($alert[0], { opacity: 0, y: -12, duration: 0.3, onComplete: function () { $alert.remove(); } });
      }
    }, 4000);
  };

  App.Views.Notify._show = function (msg, type, icon) {
    var html = template({ message: msg, type: type, icon: icon });
    $('#notify-region').html(html);
    App.Animations.slideIn('#notify-region [role="alert"]', 'down');
    setTimeout(function () {
      var $alert = $('#notify-region [role="alert"]');
      if ($alert.length) {
        gsap.to($alert[0], { opacity: 0, y: -12, duration: 0.3, onComplete: function () { $alert.remove(); } });
      }
    }, 2500);
  };

})();

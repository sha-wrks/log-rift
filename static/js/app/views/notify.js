;(function () {

  var template = Handlebars.compile($('#tpl-notify').html());

  App.Views.Notify = {};

  App.Views.Notify.success = function (msg) {
    App.Views.Notify._show(msg, 'success', 'fa-check-circle');
  };

  App.Views.Notify.error = function (msg) {
    App.Views.Notify._show(msg, 'danger', 'fa-exclamation-triangle');
  };

  App.Views.Notify._show = function (msg, type, icon) {
    var html = template({ message: msg, type: type, icon: icon });
    $('#notify-region').html(html);
    setTimeout(function () {
      $('#notify-region .alert').fadeOut(400, function () { $(this).remove(); });
    }, 2500);
  };

})();

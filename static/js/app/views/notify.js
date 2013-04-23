;(function () {

  App.Views.Notify = {};

  App.Views.Notify.success = function (msg) {
    App.Views.Notify._show(msg, 'alert-success');
  };

  App.Views.Notify.error = function (msg) {
    App.Views.Notify._show(msg, 'alert-danger');
  };

  App.Views.Notify._show = function (msg, type) {
    var $alert = $('<div class="alert ' + type + ' alert-dismissible fade in" role="alert">' +
      '<button type="button" class="close" data-dismiss="alert">&times;</button>' +
      msg + '</div>').hide();
    $('.container').first().prepend($alert);
    $alert.slideDown(200);
    setTimeout(function () { $alert.slideUp(400, function () { $(this).remove(); }); }, 2500);
  };

})();

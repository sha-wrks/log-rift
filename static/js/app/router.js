;(function () {

  App.Router = Backbone.Router.extend({

    routes: {
      '':        'list',
      'task/:id': 'detail'
    },

    list: function () {
      if (App.detailView) {
        App.detailView.remove();
        App.detailView = null;
      }
      if (App.listView) {
        App.listView.render();
      }
      App.tasks.fetch({
        reset: true,
        success: function () {
          if (App.tasks.length > 0 && App.tasks.pending().length === 0) {
            if (typeof App.confetti === 'function') App.confetti();
          }
        }
      });
    },

    detail: function (id) {
      var task = App.tasks.get(id);
      if (!task) {
        task = new App.Models.Task({ id: id });
        task.fetch();
      }
      if (App.detailView) {
        App.detailView.remove();
      }
      App.detailView = new App.Views.TaskDetail({ model: task });
      App.detailView.render();
    }
  });

})();

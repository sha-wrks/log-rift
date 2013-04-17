;(function () {

  App.Views.TaskList = Backbone.View.extend({

    el: '#app',

    template: Handlebars.compile($('#tpl-app').html()),

    initialize: function () {
      this.collection = App.tasks;
      this.listenTo(this.collection, 'reset', this.render);
      this.listenTo(this.collection, 'add', this.render);
      this.listenTo(this.collection, 'change', this.render);
      this.listenTo(this.collection, 'destroy', this.render);
    },

    render: function () {
      this.$el.html(this.template());
      this.renderProgress();
      this.renderList();
      return this;
    },

    renderProgress: function () {
      var sub = new App.Views.Progress({ collection: this.collection });
      $('#progress-region').html(sub.render().el);
    },

    renderList: function () {
      var pending = this.collection.pending();
      var done    = this.collection.done();
      var listHtml = Handlebars.compile($('#tpl-task-list').html())({
        pendingCount: pending.length,
        doneCount: done.length
      });
      $('#list-region').html(listHtml);

      var self = this;
      if (pending.length === 0 && done.length === 0) {
        $('#pending-list').html(Handlebars.compile($('#tpl-empty').html())());
      } else {
        _.each(pending, function (t) { self._appendItem('#pending-list', t); });
        _.each(done, function (t) { self._appendItem('#done-list', t); });
      }

      $('#form-quick-add').on('submit', function (e) {
        e.preventDefault();
        var title = $('#quick-title').val().trim();
        if (!title) return;
        self.collection.create({ title: title }, {
          wait: true,
          success: function () { $('#quick-title').val('').focus(); }
        });
      });
    },

    _appendItem: function (region, model) {
      var view = new App.Views.TaskItem({ model: model });
      $(region).append(view.render().el);
    }
  });

})();

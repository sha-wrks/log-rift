;(function () {

  App.Views.TaskItem = Backbone.View.extend({

    tagName: 'div',

    className: 'task-item-container',

    events: {
      'change .task-toggle': 'toggle',
      'click .task-delete': 'deleteTask',
      'click .task-title': 'navigate',
      'click .task-view': 'navigate'
    },

    template: Handlebars.compile($('#tpl-task-item').html()),

    initialize: function () {
      this.listenTo(this.model, 'change', this.render);
      this.listenTo(this.model, 'destroy', this.remove);
    },

    render: function () {
      var m = this.model;
      var priority = m.get('priority');
      var data = {
        id: m.id,
        title: m.get('title'),
        category: m.get('category'),
        done: m.get('status') === 'done',
        priorityClass: priority === 2 ? 'urgent' : priority === 1 ? 'high' : 'normal',
        priorityLabel: priority === 2 ? 'Urgent' : priority === 1 ? 'High' : null
      };
      this.$el.html(this.template(data));
      return this;
    },

    toggle: function () {
      this.model.toggle();
    },

    deleteTask: function (e) {
      e.stopPropagation();
      if (confirm('Delete this task?')) {
        this.model.destroy({ wait: true });
      }
    },

    navigate: function (e) {
      e.preventDefault();
      Backbone.history.navigate('task/' + this.model.id, { trigger: true });
    }
  });

})();

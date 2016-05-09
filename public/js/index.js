$(document).ready(function () {
	var datasource = {
		loaded: false,
		blogs: []
		};
	var content = new Vue({
		el: '#blog_list',
		data: datasource
	});
	$.get("/api/list_blog", function (data, status) {
		if (status == "success"
			&& data.code == 0) {
			content.loaded = true;
			content.blogs = data.list;
		}
	});
});

$(document).ready(function() {
	$.get("/api/list_blog", function(data, status) {
		if (status == "success"
		   && data.code == 0) {
			new Vue({
				el: '#blog_list',
				data: {
					blogs: data.list
				}
			});
		}
	});
});

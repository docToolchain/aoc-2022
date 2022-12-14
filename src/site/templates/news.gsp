<!DOCTYPE html>
<html lang="en">
<%include 'header.gsp'%>
<body onload="prettyPrint()" >
<div id="wrap">

	<%include "menu.gsp"%>
	<div class="container content">
	<div class="page-header">
		<h1>Optional Challenges</h1>
<div>
    <%
    alltags.sort().each { tag ->
        tag = tag.trim()
        def postsCount = posts.findAll { post ->
            post.status == "published" && post.tags?.contains(tag)
        }.size()
    %>
    <span><a href="${config.site_contextPath}tags/${tag.replace(' ', '-')}.html">${tag}&nbsp;<span class="badge">${postsCount}</span></a></span>

    <%
    }
    %>

</div>
	</div>
	<%published_posts.each {post ->%>
		<a href="${config.site_contextPath}/${post.uri}"><h2>${post.title}</h2></a>
		<p>${new java.text.SimpleDateFormat("dd MMMM yyyy", Locale.ENGLISH).format(post.date)}</p>
		<!--p>${post.body}</p-->
  	<%}%>
	
	<hr />
	

</div>
<div id="push"></div>
</div>

<%include "footer.gsp"%>

</body>
</html>

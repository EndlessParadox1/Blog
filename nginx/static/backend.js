let path = location.pathname;
let header = $('#header');
let content = $('#content');
let table = '<div class="mb-2">' +
    '<label for="title" class="form-label">Title</label>' +
    '<input type="text" id="title" class="form-control" placeholder="60 general characters at most" maxlength="60">' +
    '<label for="summary" class="form-label">Summary</label>' +
    '<textarea id="summary" class="form-control" placeholder="300 general characters at most" maxlength="300" rows="3"></textarea>' +
    '<label for="markdown" class="form-label">Content</label>' +
    '<textarea id="markdown" class="form-control" placeholder="Please write in Markdwon format" rows="15"></textarea></div>' +
    '<div style="text-align: center">';

$(async () => {
    let res = await fetch('/api' + path, {credentials: 'include'});
    if(!res.ok) {
        let err = await res.json();
        alert(err.msg);
    } else {
        $('#home').attr('href', path.replace('admin', 'user'));
        header.html('Hi, ' + path.replace('/admin/', ''));
        $('#logout').click(async () => await logout());
        $('#new').click(() => new_());
        $('#all').click(async () => await list());
    }
})

async function logout() {
    let ans = confirm('Sure to sign out?');
    if (ans) {
        let res = await fetch(`/api${path}/logout`, {credentials: 'include'});
        if(!res.ok) {
            let err = await res.json();
            alert('Logout failed: ' + err.msg);
        } else
            location.href = '/login.html';
    }
}

function new_() {
    header.html('New Blog');
    content.html(table + '<button id="postBtn" class="btn btn-primary">Post</button></div>');
    $('#postBtn').click(async () => {
        let ans = confirm('Sure to post?');
        if (ans) {
            let title = $('#title').val();
            let summary = $('#summary').val();
            let markdown = $('#markdown').val();
            if (!/^[\x20-\x7E]{1,60}$/.test(title) || !/^[\x20-\x7E]{1,300}$/.test(summary) || markdown === '') {
                alert('Some field is wrong!');
                return;
            }
            let res = await fetch(`/api${path}/topic`, {
                method: 'POST',
                headers: {'Content-Type': 'application/json'},
                credentials: 'include',
                body: JSON.stringify({
                    'title': title,
                    'summary': summary,
                    'markdown': markdown
                })
            });
            if(!res.ok) {
                let err = res.json();
                alert('Post failed: ' + err.msg);
            }
            else
                await list();
        }
    });
}

async function list() {
    header.html('All Blogs');
    content.html('');
    let res = await fetch(`/api${path}/topic`, {credentials: 'include'});
    let data = await res.json();
    if(!res.ok)
        alert('List failed: ' + data.msg);
    else {
        if (!$.isEmptyObject(data)) {
            let tmp = '<table class="table table-striped table-hover">' +
                '<thead style="text-align: center"><tr><th>No.</th><th>Title</th><th>Opration</th><th>Detail</th></tr></thead>' +
                '<tbody id="body"></tbody></table>';
            content.html(tmp);
            let i = 1;
            let ids = data.ids;
            let titles = data.titles;
            let body = $('#body');
            for (let k = 0; k < ids.length; k++) {
                tmp = `<tr class="trow"><td>#${i}</td><td>${titles[k]}</td>` +
                    `<td><div class="btn btn-info btn-sm" id="E${ids[k]}">Modify</div>` +
                    `<div class="btn btn-danger btn-sm" id="D${ids[k]}">Delete</div></td>` +
                    `<td><div id="B${ids[k]}" class="btn btn-light btn-sm">▲</div></td></tr>` +
                    `<tr><td colspan="4" id="C${ids[k]}" class="detail" data-flag="0"></td></tr>`;
                body.append(tmp);
                $('#E' + ids[k]).click(async () => await edit(ids[k]));
                $('#D' + ids[k]).click(async () => await del(ids[k]));
                $('#B' + ids[k]).click(async () => await detail(ids[k]));
                i++;
            }
        } else
            content.html('There is nothing yet.');
    }
}

async function edit(i) {
    header.html('Modify Blog');
    let res = await fetch(`/api/topic/${i}?level=1`);
    let data = await res.json();
    if(!res.ok)
        alert('Load failed: ' + data.msg);
    else {
        content.html(table + '<button id="submitBtn" class="btn btn-primary">Submit</button></div>');
        let title = $('#title');
        let summary = $('#summary');
        let markdown = $('#markdown');
        title.val(data.title);
        summary.val(data.summary);
        markdown.val(data.markdown);
        $('#submitBtn').click(async () => {
            let ans = confirm('Sure to submit?');
            if(ans) {
                let title = $('#title').val();
                let summary = $('#summary').val();
                let markdown = $('#markdown').val();
                if (!/^[\x20-\x7E]{1,60}$/.test(title) || !/^[\x20-\x7E]{1,300}$/.test(summary) || markdown === '') {
                    alert('Some field is wrong!');
                    return;
                }
                let res_ = await fetch(`/api${path}/topic/${i}`, {
                    method: 'PUT',
                    headers: {'Content-Type': 'application/json'},
                    credentials: 'include',
                    body: JSON.stringify({
                        'title': title,
                        'summary': summary,
                        'markdown': markdown
                    })
                });
                if(!res_.ok) {
                    let err = await res_.json();
                    alert('Update failed: ' + err.msg);
                }
                else
                     await list();
            }
        });
    }
}

async function del(i) {
    let ans = confirm('Sure to delete?');
    if(ans) {
        let res = await fetch(`/api${path}/topic/${i}`, {
            method: 'DELETE',
            credentials: 'include'
        });
        if(!res.ok) {
            let err = await res.json();
            alert('Delete failed: ' + err.msg);
        } else
            await list();
    }
}

async function detail(i) {
    let div = $('#C' + i);
    let btn = $('#B' + i);
    div.slideToggle('fast');
    if(btn.html() === '▲')
        btn.html('▼')
    else
        btn.html('▲');
    if(div.attr('data-flag') === '0') {
        let res = await fetch(`/api/topic/${i}?level=0`);
        let data = await res.json();
        if(!res.ok)
            alert('Load failed: ' + data.msg);
        else {
            div.html(data.html);
            div.attr('data-flag', '1');
        }
    }
}

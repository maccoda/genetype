#[derive(Debug)]
struct Project {
    name: String,
    description: String,
    web_url: String,
    avatar_url: Option<String>,
    git_ssh_url: String,
    git_http_url: String,
    namespace: String,
    visibility_level: i32,
    path_with_namespace: String,
    default_branch: String,
    homepage: String,
    url: String,
    ssh_url: String,
    http_url: String,
}

#[derive(Debug)]
struct Repository {
    name: String,
    url: String,
    description: String,
    homepage: String,
    git_http_url: String,
    git_ssh_url: String,
    visibility_level: i32,
}

#[derive(Debug)]
struct Author {
    name: String,
    email: String,
}

#[derive(Debug)]
struct Commit {
    id: String,
    message: String,
    timestamp: String,
    url: String,
    author: Author,
    added: Vec<String>,
    modified: Vec<String>,
    removed: Vec<String>,
}

#[derive(Debug)]
struct GitlabPush {
    object_kind: String,
    before: String,
    after: String,
    refs: String,
    checkout_sha: String,
    user_id: i32,
    user_name: String,
    user_username: String,
    user_email: String,
    user_avatar: String,
    project_id: i32,
    project: Project,
    repository: Repository,
    commits: Vec<Commit>,
    total_commits_count: i32,
}
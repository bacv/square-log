local github_sources = {
    {
        id = "gh_activity",
        interval = "1h",
        user = "bacv",
        auth = "auth token",
        user_agent = "bacv" -- should be the github username.
    }
}

function sq_sources_fn()
    return {
        github_user_activity = github_sources
    }
end

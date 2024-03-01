local API_FORMAT = "https://api.github.com/users/%s/events?page=%d"

local function parseGitHubTimestamp(timestamp)
    local pattern = "(%d+)%-(%d+)%-(%d+)T(%d+):(%d+):(%d+)Z"
    local year, month, day, hour, min, sec = timestamp:match(pattern)
    local parsedTime = {
        year = tonumber(year),
        month = tonumber(month),
        day = tonumber(day),
        hour = tonumber(hour),
        min = tonumber(min),
        sec = tonumber(sec),
    }
    return os.time(parsedTime)
end

local function convertEventToRecords(event)
    local records = {}
    local title = ""
    local tags = { event.type }
    local description = event.repo.name
    local link = event.repo.url
    local hash = tostring(event.id)
    local origin_timestamp = parseGitHubTimestamp(event.created_at)
    local pull_timestamp = os.time()

    if event.type == "PushEvent" then
        for _, commit in ipairs(event.payload.commits) do
            title = commit.message
            table.insert(tags, "Commit")
            local record = {
                title = title,
                description = description,
                tags = tags,
                link = commit.url,
                extended = {},
                hash = tostring(commit.sha),
                origin_timestamp = origin_timestamp,
                pull_timestamp = pull_timestamp,
            }
            table.insert(records, record)
        end
    elseif event.type == "PullRequestEvent" then
        title = "PR: " .. event.payload.pull_request.title
        table.insert(tags, "Pull Request")
        table.insert(tags, event.payload.action) -- e.g. "opened", "closed"
        local record = {
            title = title,
            description = description,
            tags = tags,
            link = event.payload.pull_request.html_url,
            extended = {},
            hash = hash,
            origin_timestamp = origin_timestamp,
            pull_timestamp = pull_timestamp,
        }
        table.insert(records, record)
    elseif event.type == "IssueCommentEvent" then
        title = "Comment on Issue: " .. event.payload.issue.title
        table.insert(tags, "Comment")
        table.insert(tags, "Issue")
        local record = {
            title = title,
            description = description,
            tags = tags,
            link = event.payload.comment.html_url,
            extended = {},
            hash = hash,
            origin_timestamp = origin_timestamp,
            pull_timestamp = pull_timestamp,
        }
        table.insert(records, record)
    elseif event.type == "PullRequestReviewCommentEvent" then
        title = "Review Comment on PR: " .. event.payload.pull_request.title
        table.insert(tags, "Review")
        table.insert(tags, "Pull Request")
        local record = {
            title = title,
            description = description,
            tags = tags,
            link = event.payload.comment.html_url,
            extended = {},
            hash = hash,
            origin_timestamp = origin_timestamp,
            pull_timestamp = pull_timestamp,
        }
        table.insert(records, record)
    elseif event.type == "CreateEvent" then
        local ref_type = tostring(event.payload.ref_type) or ""
        local ref = tostring(event.payload.ref) or ""

        title = "Created " .. ref_type .. ": " .. ref
        table.insert(tags, "Create")
        if ref_type ~= nil then
            table.insert(tags, event.payload.ref_type) -- e.g. "branch", "tag"
        end

        local record = {
            title = title,
            description = description,
            tags = tags,
            link = link,
            extended = {},
            hash = hash,
            origin_timestamp = origin_timestamp,
            pull_timestamp = pull_timestamp,
        }
        table.insert(records, record)
    else
        -- Fallback for unhandled event types.
        title = event.type .. " by " .. event.actor.login
        local record = {
            title = title,
            description = description,
            tags = tags,
            link = link,
            extended = {},
            hash = hash,
            origin_timestamp = origin_timestamp,
            pull_timestamp = pull_timestamp,
        }
        table.insert(records, record)
    end

    return records
end

local headers = {}

sq_pull_fn = function(source)
    local username = source.user
    local auth = source.auth
    local lastStoredEvent = sq_log.db:get_latest()
    local lastStoredEventId = lastStoredEvent and lastStoredEvent["hash"] or nil

    headers["authorization"] = "Bearer" .. auth
    headers["user-agent"] = source.user_agent
    headers["accept"] = "application/vnd.github+json"
    headers["X-GitHub-Api-Version"] = "2022-11-28"

    local page = 1
    local shouldContinue = true

    while shouldContinue do
        local url = string.format(API_FORMAT, username, page)
        local response = sq_log.http:fetch_json(url, headers)
        if not response or #response == 0 then
            shouldContinue = false
            break
        end

        for _, event in ipairs(response) do
            if tostring(event.id) == lastStoredEventId then
                shouldContinue = false
                break
            end

            -- One event could have multiple subevents (like commits).
            local records = convertEventToRecords(event)
            for _, record in ipairs(records) do
                sq_log.db:insert_data(record)
            end
        end

        if shouldContinue then
            page = page + 1
        end
    end
end

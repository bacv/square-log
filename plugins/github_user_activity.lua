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

local function convertEventToRecord(event)
    local tags = { event.type }

    if event.type == "PullRequestEvent" then
        table.insert(tags, "Pull Request")
        table.insert(tags, event.payload.action) -- "opened", "closed"
    elseif event.type == "IssueCommentEvent" then
        table.insert(tags, "Comment")
        table.insert(tags, "Issue")
    elseif event.type == "PullRequestReviewCommentEvent" then
        table.insert(tags, "Review")
        table.insert(tags, "Pull Request")
    elseif event.type == "CreateEvent" and event.payload.ref_type == "repository" then
        table.insert(tags, "New Repository")
    end

    local record = {
        title = event.type .. " by " .. event.actor.login,
        description = event.repo.name,
        tags = tags,
        link = event.repo.url,
        extended = {},
        hash = tostring(event.id),
        origin_timestamp = parseGitHubTimestamp(event.created_at),
        pull_timestamp = os.time(),
    }
    return record
end

local headers = {}

sq_pull_fn = function(source)
    local username = source.user
    local auth = source.auth
    local lastStoredEvent = sq_log.db:get_latest()
    local lastStoredEventId = lastStoredEvent and lastStoredEvent["hash"] or nil

    headers["authorization"] = "Bearer" .. auth
    headers["user-agent"] = "square-log"

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

            local record = convertEventToRecord(event)
            sq_log.db:insert_data(record)
        end

        if shouldContinue then
            page = page + 1
        end
    end
end

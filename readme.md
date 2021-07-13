# Doitlater

This is a very simple background job queue implementation using Redis as the storage medium. It, as of now, supports simple one-shot tasks without retries and a cron-like scheduled jobs, note however that a failure of an instance may break the job scheduling.
## Usage

The best way how to learn the usage patterns are likely the examples in this repository. Note that for now you must include the typetag dependency explicitly, if someone finds a way how to reexport the macro without breaking it, a PR will be appreciated.
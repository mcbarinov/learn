--> Run a script
psql -d my_database -f /path/to/script.sql

--> Install on macOS
brew install postgresql

--> List open connections / sessions
select pid as process_id, 
       usename as username, 
       datname as database_name, 
       client_addr as client_address, 
       application_name,
       backend_start,
       state,
       state_change
from pg_stat_activity;

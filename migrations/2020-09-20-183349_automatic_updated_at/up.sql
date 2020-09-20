-- from: https://x-team.com/blog/automatic-timestamps-with-postgresql/ --
CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
-- hook trigger --
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON recipe
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();
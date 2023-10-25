USE tapas;

CREATE TABLE datapoints (
	id INT AUTO_INCREMENT UNIQUE,
	data varchar(255),
	tags varchar(255),
	datetime INT,
	data_key BIGINT UNSIGNED UNIQUE,
	PRIMARY KEY(id)
)

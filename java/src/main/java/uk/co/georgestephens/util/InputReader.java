package uk.co.georgestephens.util;

import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;

public class InputReader {
	
	//TODO 1 Make get file as single string
	public static String getFileAsString(String filePath) throws IOException {
		System.out.println("Getting file: " + filePath);
		BufferedReader br = new BufferedReader(new FileReader(filePath));
		StringBuilder sb = new StringBuilder();
		try {
		    String line = br.readLine();

		    while (line != null) {
		        sb.append(line);
		        sb.append(System.lineSeparator());
		        line = br.readLine();
		    }
			return sb.toString();
		} finally {
		    br.close();
		}
	}
	
	//TODO 1 Make get file as ArrayList

}
